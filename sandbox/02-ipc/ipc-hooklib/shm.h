
#ifndef HOOKLIB_IPC_SHM
#define HOOKLIB_IPC_SHM
#ifdef __cplusplus
extern "C" {
#endif
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

#ifdef __cplusplus
}
#endif
#endif // HOOKLIB_IPC_SHM
