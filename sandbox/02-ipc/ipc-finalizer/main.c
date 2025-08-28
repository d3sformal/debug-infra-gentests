#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include <stdint.h>
#include <stdio.h>
#define CFG_MANUAL_INIT_DEINIT

extern int init_finalize_after_crash(const char* name_full_sem, uint32_t buff_count);

int main(int argc, char** argv) {
  if (argc != 3) {
    printf("Invalid arguments, usage: ipc-fin FULL_SEMAPHORE_NAME BUFFER_COUNT\n");
    return 2;
  }
  const char* name = argv[1];
  const char* num_str = argv[2];
  if (strlen(num_str) == 0 || strlen(num_str) > 6) { // arbitrary limit
    printf("Invalid number format of %s\n", num_str);
    return 2;
  }
  for (size_t i = 0; i < strlen(num_str); ++i) {
    if (!isdigit(num_str[i])) {
      printf("%s not a number\n", num_str);
      return 2;
    } 
  }

  uint32_t buff_count = (uint32_t)atoi(num_str);
  printf("Finalizing...\n");
  if (init_finalize_after_crash(name, buff_count) != 0) {
    printf("Failed to finalize comms\n");
  }
  return 0;
}