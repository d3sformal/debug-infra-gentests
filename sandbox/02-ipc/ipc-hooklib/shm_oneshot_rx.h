#ifndef LLCAP_ONESHOT_CHNL
#define LLCAP_ONESHOT_CHNL
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

bool oneshot_shm_read(const char *data_sem_name, const char *ack_sem_name,
                      const char *shm_name, void *target, size_t size);
#endif // LLCAP_ONESHOT_CHNL