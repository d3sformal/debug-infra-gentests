#ifndef VSTR_HOOK_LIB
#define VSTR_HOOK_LIB

#include <assert.h>
#include <cstdint>

#ifdef __cplusplus
#include <string>
#endif

#ifdef __cplusplus
extern "C" {
static_assert(sizeof(long long) == 8, "Expecting long long to be 8 bytes");
static_assert(sizeof(int) == 4, "Expecting int to be 4 bytes");
#endif

#define GENFNDECLTEST(name, argt, argvar)                                      \
  void name(argt argvar, argt *target, uint32_t module, uint32_t fn)

void hook_start(uint32_t module_id, uint32_t id);
void hook_arg_preabmle(uint32_t module_id, uint32_t fn_id);

GENFNDECLTEST(hook_int32, int, a);

#define LLONG long long // to avoid spaces in a macro
GENFNDECLTEST(hook_int64, LLONG, a);

GENFNDECLTEST(hook_float, float, a);
GENFNDECLTEST(hook_double, double, a);

GENFNDECLTEST(hook_short, short, a);
GENFNDECLTEST(hook_char, char, a);

#define UCHAR unsigned char
#define USHORT unsigned short
#define UINT unsigned int
#define ULLONG unsigned long long

GENFNDECLTEST(hook_uchar, UCHAR, a);
GENFNDECLTEST(hook_ushort, USHORT, a);
GENFNDECLTEST(hook_uint32, UINT, a);
GENFNDECLTEST(hook_uint64, ULLONG, a);

// C++ types
#ifdef __cplusplus
void vstr_extra_cxx__string(std::string *str, std::string **target,
                            uint32_t module, uint32_t function);
}
#endif

#endif // VSTR_HOOK_LIB