#include "hook.h"
#include "./shm.h"
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <stdint.h>
#include <string>

static_assert(sizeof(int) == 4, "Expecting int to be 4 bytes");
static_assert(sizeof(long long) == 8, "Expecting long long to be 8 bytes");

#define GENFN(name, argt, argvar, msg)                                         \
  GENFNDECL(name, argt, argvar) { printf("[HOOK] " msg, argvar); }

#define GENFN_PUSH(name, argt, argvar, msg)                                    \
  GENFNDECL(name, argt, argvar) {                                              \
    printf("[HOOK] " msg, argvar);                                             \
    push_data(&argvar, sizeof(argt));                                          \
  }

#define GENFN_PUSHEX(name, argt, argvar, msg, dflt)                                    \
  GENFNDECLEX(name, argt, argvar) {                                              \
    printf("[HOOK] " msg, argvar);\
    printf("[TGTA] %p\n", (void*)target);                                             \
    *target = (dflt); \
    push_data(&argvar, sizeof(argt));                                          \
  }

void hook_start(uint32_t module_id, uint32_t fn_id) {
  push_data(&module_id, sizeof(module_id));
  push_data(&fn_id, sizeof(fn_id));
}

void hook_arg_preabmle(uint32_t module_id, uint32_t fn_id) {
  push_data(&module_id, sizeof(module_id));
  push_data(&fn_id, sizeof(fn_id));
}

void hook_cstring(const char *str) {
  printf("[HOOK] cstring: %s\n", str);
  push_data(str, (uint32_t)strlen(str) + 1);
}

GENFN_PUSH(hook_int32, int, i, "int: %d\n")
GENFN_PUSHEX(hook_int32ex, int, i, "intex: %d\n", 333)

GENFN_PUSH(hook_int64, LLONG, d, "long long: %lld\n")
GENFN_PUSHEX(hook_int64ex, LLONG, d, "long long ex: %lld\n", 555)

GENFN_PUSH(hook_float, float, str, "float: %f\n")
GENFN_PUSH(hook_double, double, str, "double: %lf\n")

GENFN_PUSH(hook_short, short, str, "short: %d\n")
GENFN_PUSH(hook_char, char, str, "byte: %d\n")

GENFN_PUSH(hook_uchar, UCHAR, str, "unsigned byte: %u\n")
GENFN_PUSH(hook_ushort, USHORT, str, "unsigned short: %d\n")
GENFN_PUSH(hook_uint32, UINT, i, "unsigned int: %u\n")
GENFN_PUSH(hook_uint64, ULLONG, d, "unsigned long long: %llu\n")


GENFN_PUSHEX(hook_shortex, short, str, "ex short: %d\n", 444)
GENFN_PUSHEX(hook_charex, char, str, "ex byte: %d\n", 111)
GENFN_PUSHEX(hook_ucharex, UCHAR, str, "ex unsigned byte: %u\n", 222)
GENFN_PUSHEX(hook_ushortex, USHORT, str, "ex unsigned short: %d\n", 666)
GENFN_PUSHEX(hook_uint32ex, UINT, i, "ex unsigned int: %u\n", 777)
GENFN_PUSHEX(hook_uint64ex, ULLONG, d, "ex unsigned long long: %llu\n", 888)

#ifdef __cplusplus

GENFN(hook_stdstring8, const char *, str, "std::string: %s\n")

void vstr_extra_cxx__string(std::string *str) {
  uint32_t cstring_size = (uint32_t)strlen(str->c_str());
  uint32_t capacity = (uint32_t)str->capacity();
  uint64_t size = cstring_size + sizeof(capacity);
  if (str->size() > UINT32_MAX) {
    printf("Error: std::string too large");
    return;
  }
  printf("[HOOK] std::string %lu %u %s\n", size, capacity, str->c_str());
  push_data(&size, sizeof(size));
  push_data(&capacity, sizeof(capacity));
  push_data(str->c_str(), cstring_size);
}
#endif