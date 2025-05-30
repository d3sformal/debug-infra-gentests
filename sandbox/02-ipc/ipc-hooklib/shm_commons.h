#include <stdint.h>

// idea: 3 phases: call tracing, arg capture, testing
// call tracing only needs the first three parameters
//

typedef struct {
  uint32_t buff_count;
  uint32_t buff_len;
  uint32_t total_len;
  // the above required for call tracing and argument capture
  // false if zero, indicates whether we are in capture mode or not
  uint32_t mode; // required for argument capture and testing, 0 for call
                 // tracing, 1 for capture, 2 for testing
  // the below is required for only the testing phase
  // identifier of function under test
  uint32_t target_fnid;
  uint32_t target_modid;
  // false if zero, indicates whether we are inside a forked process - and
  // should prevent further forking when instrumented code (preamble) is reached
  // multiple times
  uint32_t forked;
  // number of arguments to read, should prevent argument hijacking when
  // instrumented code is reached multiple times (decrement & check if zero)
  uint32_t arg_count;
  // number of tests to be performed (number of forks to perform)
  uint32_t test_count;
} ShmMeta;
