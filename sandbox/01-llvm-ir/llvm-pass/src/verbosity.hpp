#ifndef IF_VERBOSE

bool verbose(bool Set, bool Value);
#define IF_VERBOSE if (verbose(false, false))

#endif

#ifndef IF_DEBUG

bool debug(bool Set, bool Value);
#define IF_DEBUG if (debug(false, false))

#endif