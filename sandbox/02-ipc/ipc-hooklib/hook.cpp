#include "hook.h"
#include "shm.h"
#include "shm_commons.h"
#include <cassert>
#include <csignal>
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <iostream>
#include <string>
#include <sys/poll.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/un.h>
#include <sys/wait.h>
#define ENDPASS_CODE 231

#define HOOKLIB_EC_PKT_RD 232
#define HOOKLIB_EC_WTPID 233
#define HOOKLIB_EC_CONN 234
#define HOOKLIB_EC_START 236
#define HOOKLIB_EC_PAIR 237
#define HOOKLIB_EC_RECV_PKT 238
#define HOOKLIB_EC_TX_END 239
#define HOOKLIB_EC_TX_FIN 240
#define HOOKLIB_EC_IMPL 241

#define GENFN_TEST_PRIMITIVE(name, argt, argvar)                               \
  GENFNDECLTEST(name, argt, argvar) {                                          \
    if (in_testing_mode()) {                                                   \
      /* we do not check for in_fork - if we are here, we MUST be in a forked  \
       * process - i.e. the parent of this process will stop in preamble       \
       * function and never will never reach argument instruemntation*/        \
      if (!is_fn_under_test(module, fn)) {                                     \
        goto just_copy_arg;                                                    \
      } else {                                                                 \
        if (!should_hijack_arg()) {                                            \
          goto just_copy_arg;                                                  \
        }                                                                      \
        register_argument();                                                   \
        if (!consume_bytes_from_packet(sizeof(argvar), target)) {              \
          std::cerr << "Failed to get " << sizeof(argvar) << " bytes"          \
                    << std::endl;                                              \
          perror("");                                                          \
          exit(HOOKLIB_EC_PKT_RD);                                             \
        }                                                                      \
      }                                                                        \
      return;                                                                  \
    }                                                                          \
    push_data(&argvar, sizeof(argvar));                                        \
                                                                               \
  just_copy_arg:                                                               \
    *target = argvar;                                                          \
    return;                                                                    \
  }

static int s_server_socket = -1;

static bool connect_to_server(const char *path) {
  socklen_t len;
  struct sockaddr_un remote;
  remote.sun_family = AF_UNIX;

  s_server_socket = socket(AF_UNIX, SOCK_STREAM, 0);
  if (s_server_socket == -1) {
    perror("Failed to create socket\n");
    return false;
  }
  // 108 is the limith of the sun_path field
  constexpr size_t SUN_PATH_MAX_LEN = 108;
  strncpy(remote.sun_path, path, SUN_PATH_MAX_LEN);
  len = static_cast<socklen_t>(strnlen(remote.sun_path, SUN_PATH_MAX_LEN) +
                               sizeof(remote.sun_family));
  // reinterpret_cast should be legal here... (otherwise there is only C-style
  // cast)
  if (connect(s_server_socket, reinterpret_cast<struct sockaddr *>(&remote),
              len) == -1) {
    perror("Failed to connect\n");
    return false;
  }

  return true;
}

static bool do_srv_send(void *data, size_t size, const char *desc) {
  if (send(s_server_socket, data, size, 0) == -1) {
    std::cerr << "Failed to send " << desc << std::endl;
    perror("");
    close(s_server_socket);
    return false;
  }
  return true;
}

static bool do_srv_recv(void *target, size_t size, const char *desc) {
  ssize_t rcvd = recv(s_server_socket, target, size, 0);
  if (rcvd <= 0) {
    std::cerr << "Failed to recv " << desc << ", rcvd: " << rcvd << std::endl;
    perror("");
    close(s_server_socket);
    return false;
  } else if (static_cast<size_t>(rcvd) < size) {
    std::cerr << "Failed to recv size at " << desc << ", got: " << rcvd
              << " expected " << size << '\n';
    perror("");
    close(s_server_socket);
    return false;
  }
  return true;
}

#define MSG_SIZE 16

static bool send_start_msg(uint32_t mod, uint32_t fun, uint32_t call_idx) {
  char message[MSG_SIZE];
  static_assert(sizeof(TAG_START) + sizeof(mod) + sizeof(fun) +
                    sizeof(call_idx) <=
                MSG_SIZE);
  memcpy(message, &TAG_START, sizeof(TAG_START));
  memcpy(message + 2, &mod, sizeof(mod));
  memcpy(message + 6, &fun, sizeof(fun));
  memcpy(message + 10, &call_idx, sizeof(call_idx));
  return do_srv_send(message, sizeof(message), "msg start");
}

// regardless of return type, the target must be freed by caller
static bool request_packet_from_server(uint64_t index, void **target,
                                       uint32_t *packet_size) {
  *target = NULL;
  *packet_size = 0;
  char message[MSG_SIZE];
  memcpy(message, &TAG_PKT, sizeof(TAG_PKT));
  memcpy(message + 2, &index, sizeof(index));
  if (!do_srv_send(message, sizeof(message), "pktrq")) {
    return false;
  }
  size_t pkt_size = 0;
  if (!do_srv_recv(message, sizeof(uint32_t), "pkt sz")) {
    return false;
  }
  pkt_size = static_cast<size_t>(*reinterpret_cast<uint32_t *>(message));
  void *buff = malloc(pkt_size);
  *target = buff;
  if (buff == NULL) {
    perror("Failed to alloc pkt");
    close(s_server_socket);
    return false;
  }

  if (!do_srv_recv(buff, pkt_size, "pkt data")) {
    return false;
  }
  *packet_size = static_cast<uint32_t>(pkt_size);
  return true;
}

enum class EMsgEnd : int8_t {
  MSG_END_TIMEOUT = -1,
  MSG_END_SIGNAL = -2,
  MSG_END_STATUS = 0,
  MSG_END_PASS = 1,
  MSG_END_FATAL = -64
};

static bool send_test_end_message(uint64_t index, EMsgEnd end_type,
                                  int32_t status) {
  uint16_t tag = TAG_FATAL;
  switch (end_type) {
  case EMsgEnd::MSG_END_TIMEOUT:
    tag = TAG_TIMEOUT;
    break;
  case EMsgEnd::MSG_END_SIGNAL:
    tag = TAG_SGNL;
    break;
  case EMsgEnd::MSG_END_STATUS:
    tag = TAG_EXIT;
    break;
  case EMsgEnd::MSG_END_PASS:
    tag = TAG_PASS;
    break;

  case EMsgEnd::MSG_END_FATAL:
  default:
    tag = TAG_FATAL;
    break;
  }

  char message[MSG_SIZE];
  static_assert(sizeof(TAG_TEST_END) + sizeof(index) + sizeof(tag) +
                    sizeof(status) <=
                MSG_SIZE);
  memcpy(message, &TAG_TEST_END, 2);
  memcpy(message + 2, &index, sizeof(index));
  memcpy(message + 10, &tag, sizeof(tag));
  memcpy(message + 12, &status, sizeof(status));
  return do_srv_send(message, sizeof(message), "test end msg");
}

static bool send_finish_message() {
  char message[MSG_SIZE];
  static_assert(MSG_SIZE >= sizeof(TAG_TEST_FINISH));

  memcpy(message, &TAG_TEST_FINISH, sizeof(TAG_TEST_FINISH));
  return do_srv_send(message, sizeof(message), "test finish msg");
}

void hook_start(uint32_t module_id, uint32_t fn_id) {
  push_data(&module_id, sizeof(module_id));
  push_data(&fn_id, sizeof(fn_id));
}

static bool try_wait_pid(pid_t pid, int32_t *status, EMsgEnd *result) {
  int w = waitpid(pid, status, WNOHANG | WUNTRACED | WCONTINUED);
  if (w == -1) {
    std::cerr << "Failed waitpid" << std::endl;
    exit(HOOKLIB_EC_WTPID);
  } else if (w != 0) {
    if (w != pid) {
      std::cerr << "PID does not match... " << pid << " " << w << std::endl;
    }
    if (WIFEXITED(*status)) {
      *status = WEXITSTATUS(*status);
      *result = EMsgEnd::MSG_END_STATUS;
    } else if (WIFSIGNALED(*status)) {
      *status = WTERMSIG(*status);
      *result = EMsgEnd::MSG_END_SIGNAL;
    } else if (WIFSTOPPED(*status)) {
      *status = WSTOPSIG(*status);
      *result = EMsgEnd::MSG_END_SIGNAL;
    } else if (WIFCONTINUED(*status)) {
      *status = 0;
      *result = EMsgEnd::MSG_END_SIGNAL;
    } else {
      *result = EMsgEnd::MSG_END_SIGNAL;
    }
    return true;
  }
  return false;
}

enum class PollResult : std::uint8_t {
  ResultFDReady,
  ResultTimeout,
  ResultFail
};

static PollResult do_poll(int fd, short events, int timeout_ms, int *result) {
  events = POLLERR | POLLRDHUP | events;
  pollfd pollfd = {.fd = fd, .events = events, .revents = 0};

  int rv = poll(&pollfd, 1, timeout_ms);
  if (rv == 0) {
    // timeout
    return PollResult::ResultTimeout;
  } else if (rv < 0) {
    perror("Failed to poll test rq sock");
    return PollResult::ResultFail;
  }
  if ((rv & POLLERR) != 0 || (rv & POLLRDHUP) != 0) {
    std::cerr << "FD error " << rv << '\n';
    return PollResult::ResultFail;
  }

  *result = rv;
  return PollResult::ResultFDReady;
}

enum class ERequestResult : uint8_t { Error, Continue, TestPass };

static ERequestResult handle_requests(int rq_sock) {
  int poll_rv;
  PollResult poll_res = do_poll(rq_sock, POLLIN, 50, &poll_rv);
  if (poll_res == PollResult::ResultFail) {
    return ERequestResult::Error;
  }
  if (poll_res == PollResult::ResultTimeout) {
    return ERequestResult::Continue;
  }
  if (poll_res != PollResult::ResultFDReady) {
    assert(false && "Invalid value");
  }

  if ((poll_rv & POLLIN) == 0) {
    return ERequestResult::Continue;
  }

  uint64_t packet_idx;
  if (read(rq_sock, &packet_idx, sizeof(packet_idx)) != sizeof(packet_idx)) {
    perror("read failed\n");
    return ERequestResult::Error;
  }

  if (packet_idx == HOOKLIB_TESTPASS_VAL) {
    return ERequestResult::TestPass;
  }

  void *packet_ptr;
  uint32_t packet_size;
  if (!request_packet_from_server(packet_idx, &packet_ptr, &packet_size)) {
    std::cerr << "Pktrq failed pkt idx " << packet_idx << std::endl;
    free(packet_ptr);
    return ERequestResult::Error;
  }
  if (write(rq_sock, &packet_size, sizeof(packet_size)) !=
      sizeof(packet_size)) {
    std::cerr << "Pkt sz send failed" << std::endl;
    free(packet_ptr);
    return ERequestResult::Error;
  }
  if (write(rq_sock, packet_ptr, packet_size) != packet_size) {
    std::cerr << "Pkt data send failed" << std::endl;
    free(packet_ptr);
    return ERequestResult::Error;
  }

  free(packet_ptr);
  return ERequestResult::Continue;
}

static EMsgEnd serve_for_child_until_end(int test_requests_socket, pid_t pid,
                                         int timeout_s, int32_t *status) {
  time_t seconds;
  EMsgEnd result = EMsgEnd::MSG_END_FATAL;
  seconds = time(NULL);
  while (true) {
    if (try_wait_pid(pid, status, &result)) {
      if (result == EMsgEnd::MSG_END_STATUS && *status == ENDPASS_CODE) {
        // the test could have passed due to the "special" exit code
        // (we just didnt catch it - yet)
        // we therefore check the requet socket once more in case we missed it
        switch (handle_requests(test_requests_socket)) {
        case ERequestResult::TestPass:
          return EMsgEnd::MSG_END_PASS;
        // fallthrough intended, the above can fail, we will just return the
        // result we got (status code)
        case ERequestResult::Error:
        case ERequestResult::Continue:
          break;
        }
      }
      return result;
    }

    if (time(NULL) - seconds >= timeout_s) {
      std::cerr << "\tLLCAP-TEST Timeout (" << timeout_s << " s)" << std::endl;
      return EMsgEnd::MSG_END_TIMEOUT;
    }

    switch (handle_requests(test_requests_socket)) {
    case ERequestResult::Error:
      std::cerr << "Request handler failed" << std::endl;
      return EMsgEnd::MSG_END_FATAL;
    case ERequestResult::TestPass:
      return EMsgEnd::MSG_END_PASS;
    case ERequestResult::Continue:
      break;
    }
  }
}

static void perform_testing(uint32_t module_id, uint32_t function_id,
                            uint32_t call_idx) {
  if (!connect_to_server(TEST_SERVER_SOCKET_NAME)) {
    std::cerr << "Failed to connect" << std::endl;
    exit(HOOKLIB_EC_CONN);
  }

  if (!send_start_msg(module_id, function_id, call_idx)) {
    std::cerr << "Failed send start message" << std::endl;
    exit(HOOKLIB_EC_START);
  }
  set_fork_flag();

  for (uint32_t test_idx = 0; test_idx < test_count(); ++test_idx) {
    int sockets[2];

    if (socketpair(AF_UNIX, SOCK_STREAM, 0, sockets) == -1) {
      perror("socketpair");
      exit(HOOKLIB_EC_PAIR);
    }
    int child_socket = sockets[1];
    int parent_socket = sockets[0];
    // LLCAP-SERVER <---- UNIX domain socket ----> PARENT
    // <--par_sock]-------[child_sock--> CHILD

    pid_t pid = fork();
    if (pid == 0) {
      // CHILD
      init_packet_socket(child_socket, test_idx);
      // populates "argument packet" that will be used by instrumentation
      if (!receive_packet()) {
        perror("Failed to receive argument packet\n");
        exit(HOOKLIB_EC_RECV_PKT);
      }
      // in child process, return to resume execution (start hijacking)
      return;
    }
    // PARENT
    int status = -1;
    EMsgEnd result = serve_for_child_until_end(
        parent_socket, pid, static_cast<int>(get_test_tout_secs()), &status);
    if (result != EMsgEnd::MSG_END_STATUS &&
        result != EMsgEnd::MSG_END_SIGNAL) {
      // kill the child on non-exiting result (timeout, error, ...)
      // KILL and STOP cannot be ignored
      kill(pid, SIGSTOP);
    }

    if (!send_test_end_message(test_idx, result, status)) {
      exit(HOOKLIB_EC_TX_END);
    }
  }

  if (!send_finish_message()) {
    exit(HOOKLIB_EC_TX_FIN);
  }

  exit(0);
}

void hook_arg_preabmle(uint32_t module_id, uint32_t fn_id) {
  // CONTEXT TO KEEP IN MIND:
  // we just entered an instrumented function
  if (!in_testing_mode()) {
    // we are capturing function arguments, first we inform of the function
    // itself
    push_data(&module_id, sizeof(module_id));
    push_data(&fn_id, sizeof(fn_id));
    // the rest of this function concerns only the testing mode
    return;
  }

  // in testing mode we discriminate based on the function that is under the
  // test if THIS function (the caller of hook_arg_preabmle, see context) is the
  // desired one we must furhter determine whether we are in the "right" call
  // (n-th call)
  if (!in_testing_fork() && is_fn_under_test(module_id, fn_id)) {
    // modifies call counter
    register_call();

    // should_hijack_arg becomes true as soon as the coutner updated above
    // indicates that we "should instrument this call"
    if (should_hijack_arg()) {
      perform_testing(module_id, fn_id, get_call_num());
      // PARENT process never returns from the first call to instrumented
      // function CHILD process simply continues execution, should_hijack_arg is
      // used further in the type-hijacking functions
    }
  }
}

void hook_test_epilogue(uint32_t module_id, uint32_t fn_id) {
  if (!in_testing_mode() || !in_testing_fork() ||
      !is_fn_under_test(module_id, fn_id)) {
    return;
  }

  if (!send_test_pass_to_monitor()) {
    perror("signal end to monitor");
  }

  exit(ENDPASS_CODE);
}

GENFN_TEST_PRIMITIVE(hook_float, float, n)
GENFN_TEST_PRIMITIVE(hook_double, double, n)

GENFN_TEST_PRIMITIVE(hook_char, char, c)
GENFN_TEST_PRIMITIVE(hook_uchar, UCHAR, c)
GENFN_TEST_PRIMITIVE(hook_short, short, s)
GENFN_TEST_PRIMITIVE(hook_ushort, USHORT, s)
GENFN_TEST_PRIMITIVE(hook_int32, int, i)
GENFN_TEST_PRIMITIVE(hook_uint32, UINT, i)
GENFN_TEST_PRIMITIVE(hook_int64, LLONG, d)
GENFN_TEST_PRIMITIVE(hook_uint64, ULLONG, d)

void llcap_hooklib_extra_cxx_string(std::string *str, std::string **target,
                                    uint32_t module, uint32_t function) {
  if (in_testing_mode()) {
    if (!is_fn_under_test(module, function) || !should_hijack_arg()) {
      goto move_string_to_target;
    }
    register_argument();
    *target = new std::string();
    uint32_t cstr_size = 0;
    uint32_t capacity = 0;

    if (!consume_bytes_from_packet(4, &cstr_size) ||
        !consume_bytes_from_packet(4, &capacity)) {
      perror("str fail 1\n");
      exit(HOOKLIB_EC_PKT_RD);
    }
    if (cstr_size > capacity) {
      perror("str fail2\n");
      exit(HOOKLIB_EC_IMPL);
    }
    (*target)->reserve(capacity);
    (*target)->resize(cstr_size);
    if (!consume_bytes_from_packet(cstr_size, (*target)->data())) {
      perror("str fail 3");
      exit(HOOKLIB_EC_PKT_RD);
    }
    return;
  } else {
    uint32_t cstring_size = static_cast<uint32_t>(str->size());
    uint32_t capacity = static_cast<uint32_t>(str->capacity());
    uint64_t size = cstring_size + sizeof(capacity) + sizeof(cstring_size);
    if (str->size() > UINT32_MAX) {
      perror("Error: std::string too large");
      return;
    }

    push_data(&size, sizeof(size));
    push_data(&cstring_size, sizeof(cstring_size));
    push_data(&capacity, sizeof(capacity));
    push_data(str->c_str(), cstring_size);
  }
move_string_to_target:
  *target = str;
}
