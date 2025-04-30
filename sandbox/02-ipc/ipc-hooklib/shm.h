
#ifndef HOOKLIB_IPC_SHM
#define HOOKLIB_IPC_SHM
#ifdef __cplusplus
extern "C" {
#endif
#include <stdint.h>
#include <stddef.h>
#define SHM_OK 0
#define SHM_FAIL_RET 1
#define SHM_FAIL_NORET 2

typedef struct {
  uint32_t buff_count;
  uint32_t buff_len;
  uint32_t total_len;
} ShmMeta;

int init(void) __attribute__((constructor));
void deinit(void) __attribute__((destructor()));
int push_data(const void* data, uint32_t len);

#ifdef __cplusplus
}
#endif
#endif //HOOKLIB_IPC_SHM
