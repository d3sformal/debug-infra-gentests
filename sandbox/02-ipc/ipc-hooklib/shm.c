#include "shm.h"
#include "shm_util.h"
#include "shm_write_channel.h"
#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
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
Termination protocol: see the after_crash_recovery function
*/

// shared memory data
static const char *s_shm_meta_name = "/llcap-shmmeta";
static ShmMeta s_buff_info;

static WriteChannel s_channel;

static int get_buffer_info(const char *name, ShmMeta *target) {
  int rv = -1;

  int shared_meminfo_fd = -1;
  ShmMeta *mapped_ptr;
  rv = mmap_shmem(name, (void **)&mapped_ptr, &shared_meminfo_fd,
                  sizeof(*mapped_ptr), false);
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

#define SEMPERMS (S_IROTH | S_IWOTH | S_IWGRP | S_IRGRP | S_IWUSR | S_IRUSR)

// sets up the semaphores and information required for buffer management
// if this returns 0, s_channel is ready for use
static int setup_infra(void) {
  int rv = SHM_FAIL_NORET;

  if (get_buffer_info(s_shm_meta_name, &s_buff_info) != 0) {
    printf("Could not obtain buffer info\n");
    return rv;
  }

  ChannelInfo info;
  info.buff_count = s_buff_info.buff_count;
  info.buff_len = s_buff_info.buff_len;
  info.total_len = s_buff_info.total_len;
  printf("Buffer info: cnt %u, len %u, tot %u\n", info.buff_count,
         info.buff_len, info.total_len);
  return init_write_channel_with_info("capture", "base", &info, &s_channel);
}

int init(void) {
  int rv = SHM_FAIL_NORET;
  printf("Initializing\n");

  if (setup_infra() != 0) {
    printf("Failed to init infra\n");
    exit(-1);
    return rv;
  }

  return channel_start(&s_channel);
}

int push_data(const void *source, uint32_t len) {
  return channel_write(&s_channel, source, len);
}

void deinit(void) {
  printf("Deinit!\n");
  deinit_channel(&s_channel);
  printf("deinit done\n");
  return;
}

// after a crash, there can be a buffer, that needs to be flushed
// we find this by looking at the payload length of a buffer (the first 4 bytes)
// if there is 0 -> buffer has been flushed (responsibility of the other side)
//  -> we do "nothing" and only signal on the full semaphore (to make sure the
//  other side reads a "zero-length" buffer and terminates)
// if there is non-zero -> buffer was used and not flushed (due to a crash)
//  -> we signal 2 times on the semaphore, once for the outgoing data and once
//  for the terminating message
int after_crash_recovery(void) { return deinit_channel(&s_channel); }

int init_finalize_after_crash(void) {
  int rv = SHM_FAIL_NORET;

  if (setup_infra() != 0) {
    return rv;
  }
  // notice no channel_start - we don't want to gain a free buffer at start - we
  // are trying to flush an already dirty buffer left over by the crashed
  // process
  return after_crash_recovery();
}

#ifdef MANUAL_INIT_DEINIT
#endif