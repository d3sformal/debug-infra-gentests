#ifndef VSTR_HOOK_LIB
#define VSTR_HOOK_LIB

#include <assert.h>
#include <cstdint>

#ifdef __cplusplus
#include <string>
#endif

#ifdef __cplusplus
extern "C" {
#endif
static_assert(sizeof(int) == 4, "Expecting int to be 4 bytes");
static_assert(sizeof(long long) == 8, "Expecting long long to be 8 bytes");

#define GENFNDECL(name, argt, argvar) void name(argt argvar)

void hook_start(uint32_t module_id, uint32_t id);
void hook_arg_preabmle(uint32_t module_id, uint32_t fn_id);

GENFNDECL(hook_cstring, const char *, a);
GENFNDECL(hook_int32, int, a);

#define LLONG long long // to avoid spaces in a macro
GENFNDECL(hook_int64, LLONG, a);

GENFNDECL(hook_float, float, a);
GENFNDECL(hook_double, double, a);

GENFNDECL(hook_short, short, a);
GENFNDECL(hook_char, char, a);

#define UCHAR unsigned char
#define USHORT unsigned short
#define UINT unsigned int
#define ULLONG unsigned long long

GENFNDECL(hook_uchar, UCHAR, a);
GENFNDECL(hook_ushort, USHORT, a);
GENFNDECL(hook_uint32, UINT, a);
GENFNDECL(hook_uint64, ULLONG, a);

// C++ types
#ifdef __cplusplus
void vstr_extra_cxx__string(std::string *str);
}
#endif

#endif // VSTR_HOOK_LIB