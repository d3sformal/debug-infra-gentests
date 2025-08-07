
#ifndef HOOKLIB_IPC_SHM
#define HOOKLIB_IPC_SHM
#ifdef __cplusplus
extern "C" {
#endif
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// as this is sent as an "index" value otherwise, it must me large enough to never be an index
#define HOOKLIB_TESTPASS_VAL 0xFFFFFFFFFAAFF00F
#define HOOKLIB_TESTEXC_VAL  0xFFFFFFFFFAAEE00E
#ifdef MANUAL_INIT_DEINIT
#define HOOKLIB_CTOR_ATTR 
#define HOOKLIB_DTOR_ATTR
int init_finalize_after_crash(const char *full_semaphore, uint32_t buff_count);
#else 
#define HOOKLIB_CTOR_ATTR __attribute__((constructor))
#define HOOKLIB_DTOR_ATTR __attribute__((destructor))
#endif

int init(void) HOOKLIB_CTOR_ATTR;
void deinit(void) HOOKLIB_DTOR_ATTR;

int push_data(const void *source, uint32_t len);

bool in_testing_mode(void);
bool in_testing_fork(void);
uint16_t get_test_tout_secs(void);
bool should_hijack_arg(void);
uint32_t test_count(void);
bool is_fn_under_test(uint32_t mod, uint32_t fn);

uint32_t get_call_num(void);
void register_call(void);
void register_argument(void);
void set_fork_flag(void);

bool send_test_pass_to_monitor(bool exception);
bool consume_bytes_from_packet(size_t bytes, void *target);
bool receive_packet(void);
void init_packet_socket(int fd, uint64_t request_idx);

#ifdef __cplusplus
}
#endif
#endif // HOOKLIB_IPC_SHM
