
#ifndef HOOKLIB_IPC_SHM
#define HOOKLIB_IPC_SHM
#ifdef __cplusplus
extern "C" {
#endif
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#define SHM_OK 0
#define SHM_FAIL_RET 1
#define SHM_FAIL_NORET 2

#include "shm_commons.h"

#ifdef MANUAL_INIT_DEINIT

int init(void);
void deinit(void);
int init_finalize_after_crash(void);

#else
int init(void) __attribute__((constructor));
void deinit(void) __attribute__((destructor));
#endif

int push_data(const void *data, uint32_t len);

bool in_testing_mode(void);
bool in_testing_fork(void);
bool should_hijack_arg(void);
uint32_t test_count(void);
bool is_fn_under_test(uint32_t mod, uint32_t fn);

void register_argument(void);
void set_fork_flag(void);

bool consume_bytes_from_packet(size_t bytes, void *target);
bool receive_packet(uint32_t mod, uint32_t fn);
void init_packet_socket(int fd, uint32_t request_idx);

bool report_test(uint32_t mod, uint32_t fn, uint32_t test_idx, int status,
  int result);

#ifdef __cplusplus
}
#endif
#endif // HOOKLIB_IPC_SHM
