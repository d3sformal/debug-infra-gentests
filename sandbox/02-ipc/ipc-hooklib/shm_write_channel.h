#ifndef LLCAP_SHM_WR_CHNL
#define LLCAP_SHM_WR_CHNL

#include <semaphore.h>
#include <stdint.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

#define SEMPERMS (S_IROTH | S_IWOTH | S_IWGRP | S_IRGRP | S_IWUSR | S_IRUSR)

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
  int file_descriptor;
  void *buffer_base;
  size_t current_buffer_idx;
} WriteChannel;

// name, type, info do not have to be kept alive for the lifetime of target
int init_write_channel_with_info(const char *name, const char *type,
                                 ChannelInfo *info, WriteChannel *target);

int channel_start(WriteChannel *self);

int channel_write(WriteChannel *self, const void *source, uint32_t len);

int deinit_channel(WriteChannel *self);

#ifdef __cplusplus
}
#endif

#endif // LLCAP_SHM_WR_CHNL