#include "hook.h"
#include <stdint.h>
#include <stdio.h>
#include <string>

static_assert(sizeof(int) == 4, "Expecting int to be 4 bytes");
static_assert(sizeof(long long) == 8, "Expecting long long to be 8 bytes");

#define GENFN(name, argt, argvar, msg)                                         \
  GENFNDECL(name, argt, argvar) { printf("[HOOK] " msg, argvar); }

  void hook_start(uint32_t module_id, uint32_t id) {
    printf("[HOOK %08X] start from module %x\n", id, module_id);
  }

GENFN(hook_cstring, const char *, str, "cstring: %s\n")
GENFN(hook_int32, int, i, "int: %d\n")

GENFN(hook_int64, LLONG, d, "long long: %lld\n")

GENFN(hook_float, float, str, "float: %f\n")
GENFN(hook_double, double, str, "double: %lf\n")

GENFN(hook_short, short, str, "short: %d\n")
GENFN(hook_char, char, str, "byte: %d\n")

GENFN(hook_uchar, UCHAR, str, "unsigned byte: %u\n")
GENFN(hook_ushort, USHORT, str, "unsigned short: %d\n")
GENFN(hook_uint32, UINT, i, "unsigned int: %u\n")
GENFN(hook_uint64, ULLONG, d, "unsigned long long: %llu\n")

#ifdef __cplusplus

GENFN(hook_stdstring8, const char *, str, "std::string: %s\n")

void vstr_extra_cxx__string(std::string *str) { hook_stdstring8(str->c_str()); }
#endif