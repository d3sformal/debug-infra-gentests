#include <stdio.h>
#define CFG_MANUAL_INIT_DEINIT

extern int init_finalize_after_crash(void);

int main(void) {
  printf("Finalizing...\n");
  if (init_finalize_after_crash() != 0) {
    printf("Failed to finalize comms\n");
  }
  return 0;
}