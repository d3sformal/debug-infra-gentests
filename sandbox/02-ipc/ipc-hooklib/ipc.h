#ifdef __cplusplus
extern "C" {
#endif
#include <stdint.h>
#include <stdalign.h>
#include <stdint.h>

#define IPC_OK 0
#define IPC_FAIL_RETRY 1
#define IPC_FAIL_NORETRY 2

int ipc_init(void) __attribute__((constructor));
void ipc_destroy(void) __attribute__((destructor));

int ipc_send_entry(uint32_t fn_id, uint8_t *sha256);

#ifdef __cplusplus
}
#endif