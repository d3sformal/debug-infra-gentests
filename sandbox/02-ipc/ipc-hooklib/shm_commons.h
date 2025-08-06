
#ifndef LLCAP_SHM_COMMONS
#define LLCAP_SHM_COMMONS

#ifdef __cplusplus
static_assert(sizeof(unsigned int) == 4, "expected size of u32");
static_assert(sizeof(unsigned short) == 2, "expected size of u16");
#endif

typedef struct {
  unsigned int buff_count;
  unsigned int buff_len;
  unsigned int total_len;
  // the above required for call tracing and argument capture
  
  // false if zero, indicates whether we are in capture mode or not
  unsigned int mode; // required for argument capture and testing, 0 for call
                     // tracing, 1 for capture, 2 for testing
  // the below is required for only the testing phase
  // identifier of function under test
  unsigned int target_fnid;
  unsigned int target_modid;
  // false if zero, indicates whether we are inside a forked process - and
  // should prevent further forking when instrumented code (preamble) is reached
  // multiple times
  unsigned int forked;
  // number of arguments to read, should prevent argument hijacking when
  // instrumented code is reached multiple times (decrement & check if zero)
  unsigned int arg_count;
  // number of tests to be performed (number of forks to perform)
  unsigned int test_count;
  // the number of the call of the target function to instrument
  // utitlized by decrementing this value on each call -> equality to 1
  // means the current call shall be instrumented
  // the number passed here should be "intended call 0-based index" + 2!
  unsigned int target_call_number;
  unsigned short test_timeout_seconds;
} ShmMeta;

static const unsigned short TAG_START = 0;
static const unsigned short TAG_PKT = 1;
static const unsigned short TAG_TEST_END = 2;
static const unsigned short TAG_TEST_FINISH = 3;

static const unsigned short TAG_EXC = 13;
static const unsigned short TAG_PASS = 14;
static const unsigned short TAG_TIMEOUT = 15;
static const unsigned short TAG_EXIT = 16;
static const unsigned short TAG_SGNL = 17;
static const unsigned short TAG_FATAL = 18;

static const char *const META_SEM_DATA = "/llcap-meta-sem-data";
static const char *const META_SEM_ACK = "/llcap-meta-sem-ack";
static const char *const META_MEM_NAME = "/llcap-meta-shmem";
static const char *const TEST_SERVER_SOCKET_NAME = "/tmp/llcap-test-server";

#endif // LLCAP_SHM_COMMONS
