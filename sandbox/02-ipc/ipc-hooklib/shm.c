#include "shm.h"
#include "shm_write_channel.h"
#include <assert.h>
#include <czmq.h>
#include <czmq_library.h>
#include <fcntl.h>
#include <semaphore.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <zframe.h>
#include <zmq.h>
#include <zmsg.h>
#include <zsock.h>

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

static ShmMeta s_buff_info;
static WriteChannel s_channel;
static const char *s_main_chnl_name_data = "ipc:///tmp/llcap-zmqmain-meta";
static const char *s_zmq_packet_server = "ipc:///tmp/llcap-zmqpackets";

static bool mk_msg(void *data, size_t size, zmsg_t **target) {
  zframe_t *frame = zframe_new(data, size);
  if (frame == NULL) {
    printf("Alloc err frame\n");
    return false;
  }

  zmsg_t *msg = zmsg_new();
  if (msg == NULL) {
    printf("Alloc err msg\n");
    zframe_destroy(&frame);
    return false;
  }

  if (zmsg_append(msg, &frame) != 0) {
    printf("Cannot append a frame to a message\n");
    zframe_destroy(&frame);
    zmsg_destroy(&msg);
    return false;
  }

  *target = msg;
  return true;
}

static int send_req(zsock_t *ack) {
  int dummy = 1;
  zmsg_t *msg;
  if (!mk_msg(&dummy, sizeof(dummy), &msg)) {
    printf("Req failed");
    return false;
  }

  if (zmsg_send(&msg, ack) != 0) {
    printf("Failed to send req message\n");
    zmsg_destroy(&msg);
    return false;
  }

  return true;
}

static bool get_buffer_info(ShmMeta *target) {
  bool rv = false;

  zsock_t *data = NULL;
  data = zsock_new_req(s_main_chnl_name_data);

  if (send_req(data) == -1) {
    printf("Fail request\n");
    goto fail_sock_dtor;
  }

  zmsg_t *msg = zmsg_recv(data);
  if (msg == NULL) {
    printf("Fail msg rcv - buffinfo\n");
    goto fail_sock_dtor;
  }
  zframe_t *frame = zmsg_pop(msg);
  if (frame == NULL) {
    printf("Fail frame pop\n");
    zmsg_destroy(&msg);
    goto fail_sock_dtor;
  }

  if (zframe_size(frame) != sizeof(ShmMeta)) {
    printf("Frame data size mismatch, expecterd %lu, got %lu\n",
           sizeof(ShmMeta), zframe_size(frame));
    zmsg_destroy(&msg);
    goto fail_sock_dtor;
  }

  *target = *(ShmMeta *)zframe_data(frame);
  zmsg_destroy(&msg);

  rv = true;
fail_sock_dtor:
  zsock_destroy(&data);
  return rv;
}

#define SEMPERMS (S_IROTH | S_IWOTH | S_IWGRP | S_IRGRP | S_IWUSR | S_IRUSR)

// sets up the semaphores and information required for buffer management
// if this returns 0, s_channel is ready for use
static int setup_infra(void) {
  int rv = SHM_FAIL_NORET;

  if (!get_buffer_info(&s_buff_info)) {
    printf("Could not obtain buffer info\n");
    return rv;
  }

  ChannelInfo info;
  info.buff_count = s_buff_info.buff_count;
  info.buff_len = s_buff_info.buff_len;
  info.total_len = s_buff_info.total_len;
  printf("Buffer info: cnt %u, len %u, tot %u, mod %u, fn %u, tests %u, args "
         "%u, mode %u\n",
         info.buff_count, info.buff_len, info.total_len,
         s_buff_info.target_modid, s_buff_info.target_fnid,
         s_buff_info.test_count, s_buff_info.arg_count, s_buff_info.mode);
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
  int rv = SHM_FAIL_NORET;
  if (setup_infra() != 0) {
    printf("Failed to init infra\n");
    exit(-1);
    return rv;
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

bool in_testing_mode(void) { return s_buff_info.mode == 2; }
bool in_testing_fork(void) { return s_buff_info.forked; }
uint32_t test_count(void) { return s_buff_info.test_count; }

void set_fork_flag(void) { s_buff_info.forked = true; }

// so far only the first call is hijacked
// the tool simply counts down the arguments for each argument replacement
// when count reaches zero, all arguments from the first call were instrumented
void register_argument(void) { s_buff_info.arg_count--; }
bool should_hijack_arg(void) { return s_buff_info.arg_count > 0; }

bool is_fn_under_test(uint32_t mod, uint32_t fn) {
  return in_testing_mode() && s_buff_info.target_modid == mod &&
         s_buff_info.target_fnid == fn;
}

static bool send_packet_req(zsock_t *ack, uint32_t mod, uint32_t fn) {
  uint16_t req_id = 1;
  zmsg_t *msg;
  unsigned char data[sizeof(req_id) + sizeof(mod) + sizeof(fn)];
  memcpy(data, &req_id, sizeof(req_id));
  memcpy(data + sizeof(req_id), &mod, sizeof(mod));
  memcpy(data + sizeof(req_id) + sizeof(mod), &fn, sizeof(fn));

  if (!mk_msg(data, sizeof(req_id) + sizeof(mod) + sizeof(fn), &msg)) {
    printf("Pktrq failed");
    return false;
  }

  if (zmsg_send(&msg, ack) != 0) {
    printf("Failed to send req pktrq\n");
    zmsg_destroy(&msg);
    return false;
  }

  return true;
}

static zmsg_t *s_packet_msg = NULL;
static size_t s_current_idx = 0;

bool receive_packet(uint32_t mod, uint32_t fn) {
  zsock_t *socket = NULL;
  socket = zsock_new_req(s_zmq_packet_server);

  if (!send_packet_req(socket, mod, fn)) {
    printf("Fail request packet\n");
    zsock_destroy(&socket);
    return false;
  }

  s_packet_msg = zmsg_recv(socket);
  if (s_packet_msg == NULL) {
    printf("Fail packet rcv\n");
    zsock_destroy(&socket);
    return false;
  }

  printf("packet recevied\n");

  zsock_destroy(&socket);
  return true;
}

bool report_test(uint32_t mod, uint32_t fn, uint32_t test_idx, int status,
                 int result) {
  FILE *file = NULL;
  const char *res =
      result == 0 ? "exit" : (result == -2 ? "signal" : "timeout");
  file = fopen("/tmp/llcap-test-results", "a");
  bool rv =
      fprintf(file, "%u %u %u %d %s\n", mod, fn, test_idx, status, res) > 0;
  fclose(file);
  return rv;
}

static zframe_t *get_packet_frame(void) { return zmsg_first(s_packet_msg); }

bool consume_bytes_from_packet(size_t bytes, void *target) {
  if (s_packet_msg == NULL) {
    printf("failed: packet uninitialized\n");
    return false;
  }
  if (bytes + s_current_idx > zframe_size(get_packet_frame())) {
    printf("failed: request %lu would result in packet overflow\n", bytes);
    return false;
  }

  memcpy(target, zframe_data(get_packet_frame()) + s_current_idx, bytes);
  s_current_idx += bytes;
  return true;
}