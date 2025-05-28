#ifndef LLCAP_SHM_UTIL
#define LLCAP_SHM_UTIL
#include <stdbool.h>
#include <stddef.h>
#define UNMAP_SHMEM_FLAG_TRY_ALL 1

// unmaps memory of length "len" mapped to "mem" backed by descriptor "fd" that
// was created with "name"
//
// flags = 0 or UNMAP_SHMEM_FLAG_TRY_ALL
//
// if UNMAP_SHMEM_FLAG_TRY_ALL is set, function attempts to unmap all resources
// even if some fail returns 0 on success, -1 if failure has occured
// (UNMAP_SHMEM_FLAG_TRY_ALL returns -2 if munmap failed only, -3 if every
// resource failed to be freed)
int unmap_shmem(void *mem, int fd, const char *name, size_t len,
                unsigned flags);

// maps memory info target along with a file descriptor fd
// for write permissions, pass nonzero write arg - TODO refactor
// returns 0 if both target and fd are valid resources
// returns -1 if any step failed, target and fd invalid
int mmap_shmem(const char *name, void **target, int *fd, size_t len,
               bool write);

#endif // LLCAP_SHM_UTIL