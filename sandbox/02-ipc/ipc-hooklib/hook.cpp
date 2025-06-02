#include "hook.h"
#include "shm.h"
#include "shm_commons.h"
#include <sys/poll.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/un.h>
#include <sys/wait.h>

#ifdef __cplusplus
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <string>
#else
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#endif

#ifdef __cplusplus
static_assert(sizeof(int) == 4, "Expecting int to be 4 bytes");
static_assert(sizeof(long long) == 8, "Expecting long long to be 8 bytes");
#endif

#define GENFN_TEST_PRIMITIVE(name, argt, argvar)                               \
  GENFNDECLTEST(name, argt, argvar) {                                          \
    if (in_testing_mode()) {                                                   \
      /* we do not check for in_fork - if we are here, we MUST be in a forked  \
       * process - i.e. the parent of this process will stop in preamble       \
       * function and never will never reach argument instruemntation*/        \
      if (!is_fn_under_test(module, fn)) {                                     \
        goto just_copy_arg;                                                    \
      } else {                                                                 \
        if (!should_hijack_arg()) { /* so far only the first call is hijacked  \
                                       - this is because recursion comes into  \
                                       play for multiple calls*/               \
          goto just_copy_arg;                                                  \
        }                                                                      \
        register_argument();                                                   \
        if (!consume_bytes_from_packet(sizeof(argvar), target)) {              \
          printf("Failed to get %lu bytes\n", sizeof(argvar));                 \
          perror("");                                                          \
          exit(2556);                                                          \
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
  int len;
  struct sockaddr_un remote;
  remote.sun_family = AF_UNIX;

  if ((s_server_socket = socket(AF_UNIX, SOCK_STREAM, 0)) == -1) {
    perror("Failed to create socket\n");
    return false;
  }

  strcpy(remote.sun_path, path);
  len = strlen(remote.sun_path) + sizeof(remote.sun_family);
  if (connect(s_server_socket, (struct sockaddr *)&remote, len) == -1) {
    perror("Failed to connect\n");
    return false;
  }

  return true;
}

static bool do_srv_send(void *data, size_t size, const char *desc) {
  if (send(s_server_socket, data, size, 0) == -1) {
    printf("Failed to send %s\n", desc);
    perror("");
    close(s_server_socket);
    return false;
  }
  return true;
}

static bool do_srv_recv(void *target, size_t size, const char *desc) {
  ssize_t rcvd = recv(s_server_socket, target, size, 0);
  if (rcvd <= 0) {
    printf("Failed to recv %s, rcvd: %ld\n", desc, rcvd);
    perror("");
    close(s_server_socket);
    return false;
  } else if ((size_t)rcvd < size) {
    printf("Failed to recv size at %s: got %ld expected %ld\n", desc, rcvd,
           size);
    perror("");
    close(s_server_socket);
    return false;
  }
  return true;
}

#define MSG_SIZE 16

static bool send_start_msg(uint32_t mod, uint32_t fun) {
  char message[MSG_SIZE];
  memcpy(message, &TAG_START, sizeof(TAG_START));
  memcpy(message + 2, &mod, sizeof(mod));
  memcpy(message + 6, &fun, sizeof(fun));
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
  pkt_size = (size_t)*(uint32_t *)message;
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
  *packet_size = (uint32_t)pkt_size;
  return true;
}

enum EMsgEnd {
  MSG_END_TIMEOUT = -1,
  MSG_END_SIGNAL = -2,
  MSG_END_STATUS = 0,
  MSG_END_FATAL = -64
};

static bool send_test_end_message(uint64_t index, EMsgEnd end_type,
                                  int32_t status) {
  const uint16_t tag =
      end_type == MSG_END_TIMEOUT
          ? TAG_TIMEOUT
          : (end_type == MSG_END_STATUS
                 ? TAG_EXIT
                 : (end_type == MSG_END_FATAL ? TAG_FATAL : TAG_SGNL));
  char message[MSG_SIZE];
  memcpy(message, &TAG_TEST_END, 2);
  memcpy(message + 2, &index, sizeof(index));
  memcpy(message + 10, &tag, sizeof(tag));
  memcpy(message + 12, &status, sizeof(status));
  return do_srv_send(message, sizeof(message), "test end msg");
}

static bool send_finish_message() {
  char message[MSG_SIZE];
  memcpy(message, &TAG_TEST_FINISH, 2);
  return do_srv_send(message, sizeof(message), "test finish msg");
}

void hook_start(uint32_t module_id, uint32_t fn_id) {
  push_data(&module_id, sizeof(module_id));
  push_data(&fn_id, sizeof(fn_id));
}

static bool try_wait_pid(pid_t pid, int32_t *status, EMsgEnd *result) {
  int w = waitpid(pid, status, WNOHANG | WUNTRACED | WCONTINUED);
  if (w == -1) {
    printf("Failed waitpid\n");
    exit(4411);
  } else if (w != 0) {
    if (w != pid) {
      printf("Weird, PID does not match... %d, %d\n", pid, w);
    }
    if (WIFEXITED(*status)) {
      *status = WEXITSTATUS(*status);
      *result = MSG_END_STATUS;
    } else if (WIFSIGNALED(*status)) {
      *status = WTERMSIG(*status);
      *result = MSG_END_SIGNAL;
    } else if (WIFSTOPPED(*status)) {
      *status = WSTOPSIG(*status);
      *result = MSG_END_SIGNAL;
    } else if (WIFCONTINUED(*status)) {
      *status = 0;
      *result = MSG_END_SIGNAL;
    } else {
      *result = MSG_END_SIGNAL;
    }
    return true;
  }
  return false;
}

static bool do_poll(int fd, short int events, int timeout_ms, int *result) {
  events = events | POLLERR | POLLRDHUP;
  pollfd pollfd = {.fd = fd, .events = events, .revents = 0};

  int rv = poll(&pollfd, 1, timeout_ms);
  if (rv == 0) {
    // timeout
    return true;
  } else if (rv < 0) {
    perror("Failed to poll test rq sock");
    return false;
  }

  if ((rv & POLLERR) || (rv & POLLRDHUP)) {
    printf("FD error %d\n", rv);
    return false;
  }

  *result = rv;
  return true;
}

static bool handle_requests(int rq_sock) {
  int poll_rv;
  if (!do_poll(rq_sock, POLLIN, 50, &poll_rv)) {
    return false;
  }

  if (!(poll_rv & POLLIN)) {
    return true;
  }

  uint64_t packet_idx;
  if (read(rq_sock, &packet_idx, sizeof(packet_idx)) != sizeof(packet_idx)) {
    perror("read failed\n");
    return false;
  }

  void *packet_ptr;
  uint32_t packet_size;
  if (!request_packet_from_server(packet_idx, &packet_ptr, &packet_size)) {
    printf("Pktrq failed pkt idx %lu\n", packet_idx);
    free(packet_ptr);
    return false;
  }
  if (write(rq_sock, &packet_size, sizeof(packet_size)) !=
      sizeof(packet_size)) {
    printf("Pkt sz send failed\n");
    free(packet_ptr);
    return false;
  }
  if (write(rq_sock, packet_ptr, packet_size) != packet_size) {
    printf("Pkt data send failed\n");
    free(packet_ptr);
    return false;
  }

  free(packet_ptr);
  return true;
}

static EMsgEnd serve_for_child_until_end(int test_requests_socket, pid_t pid,
                                         int timeout_s, int32_t *status) {
  time_t seconds;
  EMsgEnd result = MSG_END_FATAL;
  seconds = time(NULL);
  while (true) {
    if (try_wait_pid(pid, status, &result)) {
      return result;
    }

    if (time(NULL) - seconds >= timeout_s) {
      printf("\tTEST Timeout (%d s)", timeout_s);
      return MSG_END_FATAL;
    }

    if (!handle_requests(test_requests_socket)) {
      printf("Request handler failed\n");
      return MSG_END_FATAL;
    }
  };
  return MSG_END_FATAL;
}

static void perform_testing(uint32_t module_id, uint32_t function_id) {
  if (!connect_to_server("/tmp/llcap-test-server")) {
    printf("Failed to connect");
    exit(2389);
  }

  if (!send_start_msg(module_id, function_id)) {
    printf("Failed send start message");
    exit(2388);
  }
  set_fork_flag();

  for (uint32_t test_idx = 0; test_idx < test_count(); ++test_idx) {
    int sockets[2];

    if (socketpair(AF_UNIX, SOCK_STREAM, 0, sockets) == -1) {
      perror("socketpair");
      exit(2390);
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
        exit(3667);
      }
      // in child process, return to resume execution (start hijacking)
      return;
    }
    // PARENT
    int status = -1;
    EMsgEnd result = serve_for_child_until_end(parent_socket, pid, 3, &status);

    if (!send_test_end_message(test_idx, result, status)) {
      exit(5467);
    }
  }

  if (!send_finish_message()) {
    exit(3123);
  }

  exit(0);
}

void hook_arg_preabmle(uint32_t module_id, uint32_t fn_id) {
  if (!in_testing_mode()) {
    push_data(&module_id, sizeof(module_id));
    push_data(&fn_id, sizeof(fn_id));
    return;
  }
  if (!in_testing_fork() && is_fn_under_test(module_id, fn_id)) {
    perform_testing(module_id, fn_id);
  }
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

#ifdef __cplusplus

void vstr_extra_cxx__string(std::string *str, std::string **target,
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
      exit(3667);
    }
    if (cstr_size > capacity) {
      perror("str fail2\n");
      exit(3667);
    }
    (*target)->reserve(capacity);
    (*target)->resize(cstr_size);
    if (!consume_bytes_from_packet(cstr_size, (*target)->data())) {
      perror("str fail 3");
      exit(3667);
    }
    return;
  } else {
    uint32_t cstring_size = (uint32_t)strlen(str->c_str());
    uint32_t capacity = (uint32_t)str->capacity();
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
#endif