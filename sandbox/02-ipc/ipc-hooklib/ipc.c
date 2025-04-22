
#include "ipc.h"
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <czmq.h>
#include <unistd.h>
#include <zframe.h>
#include <zmq.h>
#include <zmsg.h>
#include <zsock.h>

// #define DEBUG

#ifdef DEBUG
#define IFDEBUG if (true)
#else
#define IFDEBUG if (false)
#endif

static zsock_t *push = NULL;
static int status = IPC_FAIL_RETRY;

int ipc_init(void) {
  if (status == IPC_OK || status == IPC_FAIL_NORETRY) {
    return status;
  }

  IFDEBUG printf("intializing \n");
  push = zsock_new_push("ipc:///tmp/zmq-socket");
  IFDEBUG printf("PUSH socket created: %p\n", (void*)push);
  sleep(1);
  status = IPC_OK;
  return IPC_OK;
}

int ipc_send_entry(uint32_t fn_id, uint8_t *sha256) {
  IFDEBUG printf("Sending to %p\n", (void*)push);
  
  zframe_t* frame = zframe_new(&fn_id, sizeof(fn_id));
  zmsg_t* msg = zmsg_new();
  
  if (zmsg_append(msg, &frame) != 0) {
    printf("Fatal error - cannot append a frame to a message\n");
    return IPC_FAIL_NORETRY;
  }
  frame = zframe_new(sha256, 64); // 64-byte hex string of 
  if (zmsg_append(msg, &frame) != 0) {
    printf("Fatal error - cannot append a frame to a message\n");
    return IPC_FAIL_NORETRY;
  }

  if (zmsg_send(&msg, push) == -1) {
    printf("Failed to send a message\n");
    return IPC_FAIL_NORETRY;
  }

  IFDEBUG printf("Sent\n");
  return IPC_OK;
}

void ipc_destroy(void) {
  if (status == IPC_OK) {
    const unsigned char DUMMY = 0;
    zframe_t* frame = zframe_new(&DUMMY, sizeof(DUMMY));
    zmsg_t* msg = zmsg_new();
  
    if (zmsg_append(msg, &frame) != 0) {
      printf("Fatal error - cannot append a frame to the final message\n");
    } else if (zmsg_send(&msg, push) == -1) {
      printf("Failed to send the final message\n");
    }

    zsock_flush(&push);
    sleep(1);
    IFDEBUG printf("Destroying socket\n");
    sleep(1);
    zsock_destroy(&push);
  }
} 