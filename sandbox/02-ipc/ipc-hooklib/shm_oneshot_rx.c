#include "shm_oneshot_rx.h"
#include "shm_util.h"
#include <fcntl.h>
#include <semaphore.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <sys/sem.h>
#include <sys/stat.h>

#define SEMPERMS (S_IROTH | S_IWOTH | S_IWGRP | S_IRGRP | S_IWUSR | S_IRUSR)

bool oneshot_shm_read(const char *data_sem_name, const char *ack_sem_name,
                      const char *shm_name, void *target, size_t size) {
  // initialize channel semaphores
  // we have 2 - the "data available" semaphore and an "ack" semaphore (signals we read the data
  // and are ready to proceed)
  sem_t *semaphore = sem_open(data_sem_name, O_CREAT, SEMPERMS, 0);
  if (semaphore == SEM_FAILED) {
    printf("Failed to initialize oneshot data semaphore %s\n", data_sem_name);
    perror("");
    return false;
  }

  sem_t *ack = sem_open(ack_sem_name, O_CREAT, SEMPERMS, 0);
  if (semaphore == SEM_FAILED) {
    printf("Failed to initialize oneshot ack semaphore %s\n", ack_sem_name);
    perror("");
    sem_close(semaphore);
    return false;
  }

  // map memory synchronized by the semaphores
  int fd = -1;
  void *source;
  if (mmap_shmem(shm_name, &source, &fd, size, false) == -1) {
    sem_close(ack);
    sem_close(semaphore);
    return false;
  }

  // wait for data to be ready
  bool rv = false;
  if (sem_wait(semaphore) == -1) {
    printf("Oneshot readout from shared memory failed on semaphore wait %s\n",
           data_sem_name);
    perror("");
    goto end;
  }
  // copy the data to the target
  memcpy(target, source, size);
  // inform we're done, cleanup
  if (sem_post(ack) != 0) {
    printf("Oneshot failed to ack on sem %s\n", ack_sem_name);
    perror("");
    goto end;
  }
  rv = true;

end:
  unmap_shmem(source, fd, shm_name, size, UNMAP_SHMEM_FLAG_TRY_ALL);
  sem_close(ack);
  sem_close(semaphore);
  return rv;
}
