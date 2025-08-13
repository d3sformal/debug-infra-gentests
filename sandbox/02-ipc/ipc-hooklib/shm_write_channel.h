#ifndef LLCAP_SHM_WR_CHNL
#define LLCAP_SHM_WR_CHNL

#include <semaphore.h>
#include <stdint.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

#define SEMPERMS (S_IROTH | S_IWOTH | S_IWGRP | S_IRGRP | S_IWUSR | S_IRUSR)

// on a higher level, channel is a simple shared memory area guarded / synchronized
// via 2 semaphores

// for "writable" channels, we implement a chunk-based approach - underlying shared memory
// is split into a same-sized chunks (called buffers), each write is either pushed inside the "current" chunk
// or (when the payload does not fit the remaining space in the chunk) the "current" chunks is
// flused, we wait for a new chunk and push the data to the new chunk (new chunk becoming the "current")

// the free and full semaphores are the consumer/producer synchronization poitns for "free" chunks
// available to the writer and "full" chunks available to the reader

// the end of the communication is a special sequence of 2*n buffer flushes, implemented by
// termination_sequence_raw

// for raw details on the inner buffer, see get_buffer_end and similar functions in the implementation

typedef struct {
  uint32_t buff_count;
  uint32_t buff_len;
  uint32_t total_len;
} ChannelInfo;

typedef struct {
  char *name_sem_free;
  char *name_sem_full;
  char *name_buff_mem;
} ChannelNames;

typedef struct {
  ChannelNames names;
  ChannelInfo info;
  uint32_t bumper_offset;
  sem_t *sem_free;
  sem_t *sem_full;
  void *buffer_base;
  size_t current_buffer_idx;
  int file_descriptor;
} WriteChannel;

// name, type, info do not have to be kept alive for the lifetime of target
// returns 0 on success
int init_write_channel_with_info(const char *name, const char *type,
                                 ChannelInfo *info, WriteChannel *target);

// initializes a channel
// returns 0 on success
int channel_start(WriteChannel *self);

// returns 0 on success
int channel_write(WriteChannel *self, const void *source, uint32_t len);

// returns 0 on success
int deinit_channel(WriteChannel *self);

// returns 0 on success
int termination_sequence_raw(sem_t* sem_full, uint32_t buffer_count);

#ifdef __cplusplus
}
#endif

#endif // LLCAP_SHM_WR_CHNL
