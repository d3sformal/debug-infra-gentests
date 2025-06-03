
#ifndef HOOKLIB_IPC_SHM
#define HOOKLIB_IPC_SHM
#ifdef __cplusplus
extern "C" {
#endif
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef MANUAL_INIT_DEINIT

int init(void);
void deinit(void);
int init_finalize_after_crash(const char *full_semaphore, uint32_t buff_count);

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

uint32_t get_call_idx(void);
void register_call(void);
void register_argument(void);
void set_fork_flag(void);

bool consume_bytes_from_packet(size_t bytes, void *target);
bool receive_packet(void);
void init_packet_socket(int fd, uint64_t request_idx);

#ifdef __cplusplus
}
#endif
#endif // HOOKLIB_IPC_SHM
