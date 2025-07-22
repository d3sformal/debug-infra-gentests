#include "shm.h"
#include "shm_commons.h"
#include "shm_oneshot_rx.h"
#include "shm_write_channel.h"
#include <assert.h>
#include <fcntl.h>
#include <semaphore.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
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
Termination protocol: see the termination_sequence_raw function
*/

static ShmMeta s_buff_info;
// should be initialized and updated such that
// - is counted down on each target fn call entry
// - never undeflows (underflow attempts are expected)
// - if == 1, then target call is reached
// => initialized to the target call number + 1
//    -> tgt call number = 1 => init to 2 => first decrement creates 1 -> hijack
static unsigned int s_call_countdown;

static WriteChannel s_channel;

static bool get_buffer_info(ShmMeta *target) {
  return oneshot_shm_read(META_SEM_DATA, META_SEM_ACK, META_MEM_NAME, target,
                          sizeof(ShmMeta));
}

// sets up the semaphores and information required for buffer management
// if this returns 0, s_channel is ready for use
static int setup_infra(void) {
  int rv = 1;

  if (!get_buffer_info(&s_buff_info)) {
    printf("Could not obtain buffer info\n");
    return rv;
  }

  s_call_countdown = s_buff_info.target_call_number + 1;

  ChannelInfo info;
  info.buff_count = s_buff_info.buff_count;
  info.buff_len = s_buff_info.buff_len;
  info.total_len = s_buff_info.total_len;
#ifdef DEBUG
  printf("Buffer info: cnt %u, len %u, tot %u, mod %u, fn %u, tests %u, args "
         "%u, mode %u\n",
         info.buff_count, info.buff_len, info.total_len,
         s_buff_info.target_modid, s_buff_info.target_fnid,
         s_buff_info.test_count, s_buff_info.arg_count, s_buff_info.mode);
#endif // DEBUG
  if (info.buff_count * info.buff_len != info.total_len) {
    printf("sanity check failed - buffer sizes\n");
    return -1;
  }

  if (in_testing_mode()) {
    return 0;
  }

  return init_write_channel_with_info("capture", "base", &info, &s_channel);
}

int init(void) {
  if (setup_infra() != 0) {
    printf("Failed to init infra\n");
    exit(-1);
  }
  if (in_testing_mode()) {
    return 0;
  }
  return channel_start(&s_channel);
}

int push_data(const void *source, uint32_t len) {
  return channel_write(&s_channel, source, len);
}

void deinit(void) {
  if (in_testing_mode()) {
    return;
  }

  deinit_channel(&s_channel);
}

#ifdef MANUAL_INIT_DEINIT
#define SEMPERMS (S_IROTH | S_IWOTH | S_IWGRP | S_IRGRP | S_IWUSR | S_IRUSR)
// after a crash, there can be a buffer, that needs to be flushed
// we find this by looking at the payload length of a buffer (the first 4 bytes)
// if there is 0 -> buffer has been flushed (responsibility of the other side)
//  -> we do "nothing" and only signal on the full semaphore (to make sure the
//  other side reads a "zero-length" buffer and terminates)
// if there is non-zero -> buffer was used and not flushed (due to a crash)
//  -> we signal 2 times on the semaphore, once for the outgoing data and once
//  for the terminating message
int init_finalize_after_crash(const char *name_full_sem, uint32_t buff_count) {
  sem_t *sem_full = sem_open(name_full_sem, O_CREAT, SEMPERMS, 0);
  if (sem_full == SEM_FAILED) {
    printf("Failed to initialize FULL semaphore %s\n", name_full_sem);
    perror("");
    return 1;
  }
  // notice no channel_start - we don't want to gain a free buffer at start - we
  // are trying to flush an already dirty buffer left over by the crashed
  // process
  return termination_sequence_raw(sem_full, buff_count);
}
#endif // MANUAL_INIT_DEINIT

bool in_testing_mode(void) { return s_buff_info.mode == 2; }
bool in_testing_fork(void) { return s_buff_info.forked; }
uint16_t get_test_tout_secs(void) { return in_testing_mode() ? s_buff_info.test_timeout_seconds : 0; }
uint32_t test_count(void) { return s_buff_info.test_count; }

void set_fork_flag(void) { s_buff_info.forked = true; }

// returns 1-based index of the current call (being/ that was) executed
uint32_t get_call_num(void) { return s_buff_info.target_call_number + 1 - s_call_countdown; }
void register_call(void) {
  if (s_call_countdown > 0) {
    // s_call_countdown 0 means, that the testing has already been performed
    // 1 means we will be testing the call that caused register_call to be called
    // otherwise "we are not at the desired call yet"
    s_call_countdown--;
  }
}
// counts down the arguments for each argument replacement
void register_argument(void) { s_buff_info.arg_count--; }
bool should_hijack_arg(void) {
  return s_call_countdown == 1 && s_buff_info.arg_count > 0;
}

bool is_fn_under_test(uint32_t mod, uint32_t fn) {
  return in_testing_mode() && s_buff_info.target_modid == mod &&
         s_buff_info.target_fnid == fn;
}

static void *s_packet = NULL;
static size_t s_current_idx = 0;
static uint32_t s_packet_size = 0;

#define PAYLOAD_T uint64_t

static int s_socket_fd = -1;
static PAYLOAD_T s_packet_idx = 0;

void init_packet_socket(int fd, PAYLOAD_T request_idx) {
  s_socket_fd = fd;
  s_packet_idx = request_idx;
}

bool receive_packet(void) {
  if (write(s_socket_fd, &s_packet_idx, sizeof(s_packet_idx)) !=
      sizeof(s_packet_idx)) {
    return false;
  }

  if (read(s_socket_fd, &s_packet_size, sizeof(s_packet_size)) !=
      sizeof(s_packet_size)) {
    perror("Failed to recv packet sz");
    return false;
  }

  s_packet = malloc(s_packet_size);
  if (s_packet == NULL) {
    perror("Failed to alloc packet");
    return false;
  }

  if (read(s_socket_fd, s_packet, s_packet_size) != s_packet_size) {
    perror("Failed to recv packet data");
    return false;
  }
  return true;
}

bool consume_bytes_from_packet(size_t bytes, void *target) {
  if (s_packet == NULL) {
    printf("failed: packet uninitialized\n");
    return false;
  }

  if (bytes > s_packet_size || (size_t)s_packet_size - bytes < s_current_idx) {
    printf("failed: request %lu would result in packet overflow (%u %lu)\n",
           bytes, s_packet_size, s_current_idx);
    return false;
  }

  memcpy(target, (const char *)s_packet + s_current_idx, bytes);
  s_current_idx += bytes;

  if (s_current_idx == s_packet_size) {
    free(s_packet);
    s_packet_size = 0;
  }
  return true;
}

bool send_test_pass_to_monitor(bool exception) {
  PAYLOAD_T payload = exception ? HOOKLIB_TESTEXC_VAL : HOOKLIB_TESTPASS_VAL;
  static_assert(sizeof(s_packet_idx) == sizeof(payload), "sanity check");

  if (write(s_socket_fd, &payload, sizeof(payload)) != sizeof(payload)) {
    return false;
  }

  return true;
}