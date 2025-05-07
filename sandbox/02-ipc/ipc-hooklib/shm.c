#include "shm.h"
#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <errno.h>
#include <fcntl.h> /* For O_* constants */
#include <semaphore.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h> /* For mode constants */
#include <unistd.h>

/*
Shared memory buffering & synchronization

In this architecture, we expect to have N buffers that are being filled and
processed in circular fashion, starting from 0. We expect a SINGLE producer and
a SINGLE consumer to coordinate their buffers.

Producer starts filling buffer 0. When it deems the buffer full, it uses 2
semaphores to:
1. signal on a "full" semaphore there is a FULL buffer to be handled
2. wait on a "free" semaphore for a FREE buffer
3. after wake-up, starts filling another buffer

Consumer starts by waiting for the "full" semaphore. As soon as it is woken up,
it:
1. processes the buffer
2. signals "free" semaphore to indicate that number of free buffers has
increased
3. waits on the "full" semaphore again

Format of a buffer:

Bytes 0-3: 4B length of payload starting from byte 4 (after this field)
Bytes 4+ : payload from the foreign process
*/

/*
Special considerations w.r.t. program **crashing**.
  - semaphore & memory should be unregistered by the OS
Termination protocol: (ensure consumer terminates as producer unexpectedly
terminates)
- separate process should connect to the "full" semaphore and:
  - signal it once (to ensure all data is "flushed" to the consumer)
  - wait on free buffer (for the next step, is valid unless consumer crashed)
  - write zero as length to the new free buffer
    - consumer should terminate and cleanup when it encounters zero-length
buffer
*/

// counts full buffers
static sem_t *s_sem_full;
static const char *s_sem_full_name = "/llcap-semfull";
// counts free buffers
static sem_t *s_sem_free;
static const char *s_sem_free_name = "/llcap-semfree";

// shared memory data
static const char *s_shm_meta_name = "/llcap-shmmeta";
static ShmMeta s_buff_info;
static int s_shared_buffers_fd = -1;
static void *s_shared_buffers_ptr = NULL;
static const char *s_shared_buffers_name = "/llcap-shmbuffs";

// bump index for a buffer
static uint32_t s_bumper = 0;

static int s_shm_initialized = SHM_FAIL_NORET;

static int close_fd(int fd, const char *fd_name, const char *fail_msg) {
  if (fd != -1) {
    if (close(fd) == -1) {
      printf("%s name: %s, fd: %d: %s\n", fail_msg, fd_name, fd, strerror(errno));
      return -1;
    }
    return 0;
  }
  return 0;
}

// maps memory info target along with a file descriptor fd
// for write permissions, pass nonzero write arg - TODO refactor
// returns 0 if both target and fd are valid resources
// returns -1 if any step failed, target and fd invalid
static int map_shmem(const char *name, void **target, int *fd, size_t len, int write) {
  int rv = -1;
  *fd = -1;

  *fd = shm_open(name, write != 0 ? O_RDWR : O_RDONLY , 0);
  if (*fd == -1) {
    printf("Failed to create shm FD for %s: %s\n", name, strerror(errno));
    return rv;
  }

  void *mem_ptr = mmap(NULL, len, write != 0 ? (PROT_READ | PROT_WRITE) : PROT_READ, MAP_SHARED, *fd, 0);
  if (mem_ptr == MAP_FAILED) {
    printf("Failed to map from FD %s: %s\n", name, strerror(errno));
    close_fd(*fd, name, "Cleanup map_shmem failed to close FD");
  } else {
    rv = 0;
    *target = mem_ptr;
  }

  return rv;
}

#define UNMAP_SHMEM_FLAG_TRY_ALL 1

// unmaps memory of length "len" mapped to "mem" backed by descriptor "fd" that
// was created with "name" flags = 0 or UNMAP_SHMEM_FLAG_TRY_ALL if
// UNMAP_SHMEM_FLAG_TRY_ALL is set, function attempts to unmap all resources
// even if some fail returns 0 on success, -1 if failure has occured
// (UNMAP_SHMEM_FLAG_TRY_ALL returns -2 if munmap failed only, -3 if every
// resource failed to be freed)
static int unmap_shmem(void *mem, int fd, const char *name, size_t len,
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

static int get_buffer_info(const char *name, ShmMeta *target) {
  int rv = -1;

  int shared_meminfo_fd = -1;
  ShmMeta *mapped_ptr;
  rv = map_shmem(name, (void **)&mapped_ptr, &shared_meminfo_fd,
                 sizeof(*mapped_ptr), 0);
  if (rv != 0) {
    return rv;
  }

  *target = *(ShmMeta *)mapped_ptr;
  rv = 0;

  // cleanup failures not considered errors
  unmap_shmem(mapped_ptr, shared_meminfo_fd, name, sizeof(*mapped_ptr),
              UNMAP_SHMEM_FLAG_TRY_ALL);
  return rv;
}

static int semaphore_close(sem_t *sem, const char *name) {
  if (sem != SEM_FAILED) {
    if (sem_close(sem) == -1) {
      printf("Failed to close semaphore %s: %s\n", name, strerror(errno));
      return -1;
    }
  }
  return 0;
}
static size_t s_writing_to_buffer_idx = 0;

// waits for a free buffer and updates related data
static int update_buffer_idx(void) {
  // wait for a free buffer
  if (sem_wait(s_sem_free) != 0) {
    printf("Failed waiting for free buffer! %s\n", strerror(errno));
    return -1;
  }

  s_bumper = 0;
  s_writing_to_buffer_idx = s_writing_to_buffer_idx + 1 == s_buff_info.buff_count ? 0 : s_writing_to_buffer_idx + 1;
  return 0;
}

// obtains a free buffer & initializes related data
// returns -1 on failure
static int init_free_buff(void) {
  s_writing_to_buffer_idx = s_buff_info.buff_count - 1;
  return update_buffer_idx();
}

#define SEMPERMS (S_IROTH | S_IWOTH | S_IWGRP | S_IRGRP | S_IWUSR | S_IRUSR)

// returns the total number of bytes in all buffers
size_t get_buff_total_sz(void) {
  return s_buff_info.buff_count * s_buff_info.buff_len;
}

// sets up the semaphores and information required for buffer management
// if this returns 0, s_buff_info, s_sem_full and s_sem_free are ready to be used
static int setup_infra(void) {
  int rv = SHM_FAIL_NORET;

  if (get_buffer_info(s_shm_meta_name, &s_buff_info) != 0) {
    return rv;
  }
  static_assert(sizeof(size_t) > sizeof(uint32_t),
  "Next line kinda depends on this");
  printf("Buffers description: cnt: %u len: %u tot: %u\n", s_buff_info.buff_count, s_buff_info.buff_len, s_buff_info.total_len);
  size_t buff_total_size = get_buff_total_sz();
  if (buff_total_size != (size_t)s_buff_info.total_len ||
      sizeof(s_bumper) >= s_buff_info.buff_len) {
    return rv;
  }

  //  If O_CREAT is specified, and a semaphore with the given name already exists, then mode and value are ignored. (this should be the case, because we will want to first run the server and then the instrumented binary)
  s_sem_full = sem_open(s_sem_full_name, O_CREAT, SEMPERMS, 0);
  if (s_sem_full == SEM_FAILED) {
    printf("Failed to initialize FULL semaphore: %s\n", strerror(errno));
    return rv;
  }

  s_sem_free = sem_open(s_sem_free_name, O_CREAT, SEMPERMS, s_buff_info.buff_count);
  if (s_sem_free == SEM_FAILED) {
    printf("Failed to initialize FREE semaphore: %s\n", strerror(errno));
    goto sem_full_cleanup;
  }

  int sem_value = -1;
  sem_getvalue(s_sem_free, &sem_value);
  printf("Semaphore value: %d\n", sem_value);
  return 0;

  sem_full_cleanup:
  semaphore_close(s_sem_full, s_sem_full_name);
  s_sem_full = SEM_FAILED;
  return rv;
}

// sets up buffer areas, completes initialization
// after this returns 0, s_shared_buffers_ptr, s_shared_buffers_fd can be used
static int setup_buffers(void) {
  size_t buff_total_size = get_buff_total_sz();
  if (map_shmem(s_shared_buffers_name, &s_shared_buffers_ptr,
                &s_shared_buffers_fd, buff_total_size, 1) != 0) {
    return -1;
  }
  return 0;
}

int init(void) {
  int rv = SHM_FAIL_NORET;
  printf("Initializing\n");

  if (setup_infra() != 0) {
    return rv;
  }

  if (setup_buffers() != 0) {
    printf("Failed to map buffer memory\n");
    s_shared_buffers_ptr =
        NULL; // so that cleanup in deinint knows what not to touch
    s_shared_buffers_fd = -1;
    goto sem_free_cleanup; // not really required but this keeps the pattern
                           // going
  } else {
    // we expect that whoever prepared the semaphores has also prepared the
    // buffers!!!
    s_shm_initialized = SHM_OK;
    printf("Init ok!\n");
    // finally, obtain one free buffer
    if (init_free_buff() == 0) {
      return 0;
    } else {
      rv = -1;
    }
  }

sem_free_cleanup:
  printf("Cleaning semaphores...\n");
  semaphore_close(s_sem_free, s_sem_free_name);
  s_sem_free = SEM_FAILED;
  semaphore_close(s_sem_full, s_sem_full_name);
  s_sem_full = SEM_FAILED;

  printf("Init returning %d\n", rv);
  if (rv != SHM_OK) {
    exit(-1);
  }

  return rv;
}

static void *get_buffer(size_t idx) {
  static_assert(sizeof(char) == 1, "Byte is a byte");
  assert(idx < s_buff_info.buff_len);
  return (void *)((char *)s_shared_buffers_ptr +
                  idx * s_buff_info.buff_len);
}

static void *get_buffer_end(void) {
  // get an offset into the payload portion of the buffer:

  //  sizeof(s_bumper) bytes      s_bumper bytes of written data
  // |----s_bumber-4Bytes----|-----------data---->
  // s_bumper is offset into data -------^^^^

  //         offset to start of data---vvvvvv-vvvvvvvv    vvvvvvvvvv----offset
  //         into the data
  return (void *)((char *)get_buffer(s_writing_to_buffer_idx) + sizeof(s_bumper) + s_bumper);
}

// used when local buffer is full and a new one is needed
static int move_to_next_buff(void) {
  if (s_shm_initialized != SHM_OK) {
    return -1;
  }
  // signal buffer is full
  if (sem_post(s_sem_full) != 0) {
    printf("Failed posting a full buffer! %s\n", strerror(errno));
    return -1;
  }

  return update_buffer_idx();
}

static uint32_t get_buff_data_space(void) {
  static_assert(sizeof(s_bumper) < (size_t)UINT32_MAX,
                "Needed for the line below");
  return s_buff_info.buff_len - (uint32_t)sizeof(s_bumper);
}

static uint32_t get_buff_data_free_space(void) {
  return get_buff_data_space() - s_bumper;
}

static int can_push_data_of_size(size_t len) {
  // check overflow
  if (SIZE_MAX - len < get_buff_data_space()) {
    printf("Overflow on data size %lu\n", len);
    return -1;
  } else if (len >= get_buff_data_space()) {
    printf("Request for data size %lu cannot be satisfied as buffer length is "
           "%u (%lu reserved)\n",
           len, s_buff_info.buff_len, sizeof(s_bumper));
    return -1;
  }

  if (len > get_buff_data_free_space()) {
    int rv = move_to_next_buff();
    if (rv != 0) {
      printf("Failed to obtain a free buffer!\n");
      return -1;
    }
    // safe recursive call as len is checked with maximum buffer size above
    // (get_buff_data_space)
    return can_push_data_of_size(len);
  }

  return 0;
}

static int unchecked_push_data(const void *source, uint32_t len) {
  void *destination = get_buffer_end();

  if ((source > destination &&
       (const char *)source < (char *)destination + len) ||
      (destination > source &&
       (char *)destination < (const char *)source + len)) {
    printf("Aliasing regions of memory when pushing data to buffer dest: %p, "
           "src: %p, len: %u\n",
           destination, source, len);
    return -1;
  }

  memcpy(destination, source, len);
  s_bumper += len;
  // in case of a crash, the last buffer's size MUST be known even if it was in progress
  *(uint32_t*)get_buffer(s_writing_to_buffer_idx) = s_bumper;
  return 0;
}

int push_data(const void *source, uint32_t len) {
  if (s_shm_initialized != SHM_OK) {
    return -1;
  }
  if (s_buff_info.buff_len <= len || can_push_data_of_size(len) != 0) {
    printf("Failed to push data due to len: %u\n", len);
    return -1;
  }

  return unchecked_push_data(source, len);
}

// terminates the protocol
int termination_sequence(void) {
  // we'll post to the "full" semaphore exactly 2 * N times (N = number of buffers)
  // this is in order to guarantee N consecutive "empty" buffers being sent
  // the above relies on the fact that the other side of the communication sets
  // the payload length (inside a buffer) to zero before "pushing it back"
  for (uint32_t i = 0; i < 2 * s_buff_info.buff_count; ++i) {
    if (sem_post(s_sem_full) != 0) {
      printf("Failed posting a full buffer! %s\n", strerror(errno));
      return -1;
    }
  }
  return 0;
}

void deinit(void) {
  printf("Deinit!\n");
  // does not have & ignores return values as the program is terminating at this
  // point
  if (move_to_next_buff() != 0) {
    printf("Failed to obtain a free buffer!\n");
  }
  
  if (termination_sequence() != 0) {
    printf("Failed to send termination sequence during deinit\n");
  }

  if (s_shared_buffers_ptr != NULL && s_shared_buffers_fd != -1) {
    unmap_shmem(s_shared_buffers_ptr, s_shared_buffers_fd,
                s_shared_buffers_name, s_buff_info.total_len,
                UNMAP_SHMEM_FLAG_TRY_ALL);
  } else if (!(s_shared_buffers_ptr == NULL && s_shared_buffers_fd == -1)) {
    printf("Shared buffers data inconsistent, unexpected values of ptr: %p, "
           "fd: %d in deinit - will NOT touch\n",
           s_shared_buffers_ptr, s_shared_buffers_fd);
  }

  semaphore_close(s_sem_free, s_sem_free_name);
  semaphore_close(s_sem_full, s_sem_full_name);
  printf("deinit done\n");
}

// after a crash, there can be a buffer, that needs to be flushed
// we find this by looking at the payload length of a buffer (the first 4 bytes)
// if there is 0 -> buffer has been flushed (responsibility of the other side)
//  -> we do "nothing" and only signal on the full semaphore (to make sure the other side reads a "zero-length" buffer and terminates)
// if there is non-zero -> buffer was used and not flushed (due to a crash)
//  -> we signal 2 times on the semaphore, once for the outgoing data and once for the terminating message
int after_crash_recovery(void) {
  // first, wait for the "free semaphore" to be N - 1 or N, N = number of buffers
  // (one is "used" or none are "used")
  int res = -1;
  printf("Recovering after crash!\n");
  do {
    sem_getvalue(s_sem_free, &res);
    printf("Semaphore value: %d\n", res);
    sleep(1);
  } while (res < 0 || (uint32_t)res < s_buff_info.buff_count - 1);
  s_shm_initialized = SHM_OK;

  if (termination_sequence() != 0) {
    printf("Failed to send termination sequence during recovery\n");
    return -1;
  }

  return 0;
}

int init_finalize_after_crash(void) {
  int rv = SHM_FAIL_NORET;

  if (setup_infra() != 0) {
    return rv;
  }

  if (setup_buffers() != 0) {
    printf("Failed to map buffer memory\n");
    s_shared_buffers_ptr =
        NULL; // so that cleanup in deinint knows what not to touch
    s_shared_buffers_fd = -1;
    goto sem_free_cleanup; // not really required but this keeps the pattern
                           // going
  } else {
    // we expect that whoever prepared the semaphores has also prepared the
    // buffers!!!
    rv = 0;
    s_shm_initialized = SHM_OK;
    printf("Init ok!\n");
    // finally, obtain one free buffer
    return after_crash_recovery();
  }

sem_free_cleanup:
  printf("Cleaning semaphore free...\n");
  semaphore_close(s_sem_free, s_sem_free_name);
  s_sem_free = SEM_FAILED;
  printf("Cleaning semaphore full...\n");
  semaphore_close(s_sem_full, s_sem_full_name);
  s_sem_full = SEM_FAILED;

  printf("Init returning %d\n", rv);
  if (rv != SHM_OK) {
    exit(-1);
  }

  return rv;
}

#ifdef MANUAL_INIT_DEINIT
#endif