#include "shm_util.h"
#include <errno.h>
#include <fcntl.h> /* For O_* constants */
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h> /* For mode constants */
#include <unistd.h>

static int close_fd(int fd, const char *fd_name, const char *fail_msg) {
  if (fd != -1) {
    if (close(fd) == -1) {
      printf("close failed: %s name: %s, fd: %d: %s\n", fail_msg, fd_name, fd,
             strerror(errno));
      return -1;
    }
    return 0;
  }
  return 0;
}

int unmap_shmem(void *mem, int fd, const char *name, size_t len,
                unsigned flags) {
  int rv = -1;
  if (munmap(mem, len) != 0) {
    printf("Failed to UNmap memory %s: %s\n", name, strerror(errno));
    if ((flags & UNMAP_SHMEM_FLAG_TRY_ALL) == 0) {
      return -1;
    } else {
      rv = -2;
    }
  }
  if (close_fd(fd, name, "Failed to close FD") != 0) {
    printf("Failed to UNlink FD %s: %s\n", name, strerror(errno));
    if (rv == -2) {
      return -3;
    }
    return -1;
  }
  return 0;
}

int mmap_shmem(const char *name, void **target, int *fd, size_t len,
               bool write) {
  int rv = -1;
  *fd = -1;

  *fd = shm_open(name, write ? O_RDWR : O_RDONLY, 0);
  if (*fd == -1) {
    printf("Failed to create shm FD for %s: %s\n", name, strerror(errno));
    return rv;
  }

  void *mem_ptr = mmap(NULL, len, write ? (PROT_READ | PROT_WRITE) : PROT_READ,
                       MAP_SHARED, *fd, 0);
  if (mem_ptr == MAP_FAILED) {
    printf("Failed to map from FD %s: %s\n", name, strerror(errno));
    close_fd(*fd, name, "Cleanup map_shmem failed to close FD");
  } else {
    rv = 0;
    *target = mem_ptr;
  }

  return rv;
}