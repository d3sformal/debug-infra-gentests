#include <assert.h>
#include <stdio.h>

static_assert(sizeof(int) == 4, "Expecting int to be 4 bytes");
static_assert(sizeof(long long) == 8, "Expecting long long to be 8 bytes");

#define GENFN(name, argt, argvar, msg)                                         \
  void name(argt argvar) { printf("[HOOK] " msg, argvar); }
GENFN(hook_start, char *, str, "start: %s\n")
GENFN(hook_int32, int, i, "int: %d\n")

#define LLONG long long // to avoid spaces in a macro
GENFN(hook_int64, LLONG, d, "long long: %lld\n")

GENFN(hook_float, float, str, "float: %f\n")
GENFN(hook_double, double, str, "double: %lf\n")
