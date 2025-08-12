#ifndef IF_VERBOSE

bool verbose(bool Set, bool Value);
#define IF_VERBOSE if (debug(false, false) || verbose(false, false))
#define VERBOSE_LOG IF_VERBOSE llvm::errs()
#endif

#ifndef IF_DEBUG

bool debug(bool Set, bool Value);
#define IF_DEBUG if (debug(false, false))
#define DEBUG_LOG IF_DEBUG llvm::errs()
#endif

#ifndef LLCAP_UTILS
#define LLCAP_UTILS

#include "llvm/Support/raw_ostream.h"
#include <charconv>
#include <optional>

template <class T, class StrT> std::optional<T> tryParse(const StrT &S) {
  T Res;
  auto [_, ec] = std::from_chars(S.data(), S.data() + S.size(), Res);

  if (ec != std::errc()) {
    VERBOSE_LOG << "Warning - invalid numeric value: " << S << '\n';
    return std::nullopt;
  }

  return Res;
}

template <class T> T valOrDefault(std::optional<T> Opt, T Default) {
  return Opt ? *Opt : Default;
}

#endif