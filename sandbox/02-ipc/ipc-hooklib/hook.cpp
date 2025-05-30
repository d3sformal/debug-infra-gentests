#include "hook.h"
#include "./shm.h"
#ifdef __cplusplus
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <string>
#else
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#endif
#include <stdint.h>
#include <sys/wait.h>
#include <unistd.h>

static_assert(sizeof(int) == 4, "Expecting int to be 4 bytes");
static_assert(sizeof(long long) == 8, "Expecting long long to be 8 bytes");

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
          printf("!!!!! Failed to get %lu bytes\n", sizeof(argvar));           \
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

void hook_start(uint32_t module_id, uint32_t fn_id) {
  push_data(&module_id, sizeof(module_id));
  push_data(&fn_id, sizeof(fn_id));
}

// 0 - ok
// -1 - timeout
// -2 - signal
static int wait_for_pid(pid_t pid, int timeout_s, int *status) {
  pid_t w;
  time_t seconds;

  seconds = time(NULL);
  do {
    if (time(NULL) - seconds >= timeout_s) {
      printf("\tTEST Timeout (%d s)", timeout_s);
      return -1;
    }
    w = waitpid(pid, status, WUNTRACED | WCONTINUED);
    if (w == -1) {
      printf("Failed waitpid");
      exit(4411);
    }

    if (WIFEXITED(*status)) {
      printf("\tTEST exited, status=%d\n", WEXITSTATUS(*status));
      return 0;
    } else if (WIFSIGNALED(*status)) {
      printf("\tTEST FAIL killed by signal %d\n", WTERMSIG(*status));
      return -2;
    } else if (WIFSTOPPED(*status)) {
      printf("\tTEST FAIL stopped by signal %d\n", WSTOPSIG(*status));
      return -2;
    } else if (WIFCONTINUED(*status)) {
      printf("\tTEST FAIL continued\n");
      return -2;
    }
    sleep(1);
  } while (!WIFEXITED(*status) && !WIFSIGNALED(*status));
  return 0;
}

static void perform_testing(uint32_t module_id, uint32_t function_id) {
  set_fork_flag();
  printf("TEST %u %u\n", module_id, function_id);
  for (uint32_t test_idx = 0; test_idx < test_count(); ++test_idx) {
    pid_t pid = fork();
    if (pid == 0) {
      if (!receive_packet(module_id, function_id)) {
        printf("Failed to receive argument packet\n");
        exit(3667);
      }
      // in child process, return to resume execution (start hijacking)
      return;
    }

    int status = -1;
    int result = wait_for_pid(pid, 3, &status);
    if (!report_test(module_id, function_id, test_idx, WEXITSTATUS(status),
                     result)) {
      printf("Test report failed\n");
      exit(4567);
    }
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
      printf("str fail 1\n");
      exit(3667);
    }
    if (cstr_size > capacity) {
      printf("str fail2\n");
      exit(3667);
    }
    (*target)->reserve(capacity);
    (*target)->resize(cstr_size);
    if (!consume_bytes_from_packet(cstr_size, (*target)->data())) {
      printf("str fail 3");
      exit(3667);
    }
    return;
  } else {
    uint32_t cstring_size = (uint32_t)strlen(str->c_str());
    uint32_t capacity = (uint32_t)str->capacity();
    uint64_t size = cstring_size + sizeof(capacity) + sizeof(cstring_size);
    if (str->size() > UINT32_MAX) {
      printf("Error: std::string too large");
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