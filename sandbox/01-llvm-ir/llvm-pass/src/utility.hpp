#ifndef LLCAP_UTILS
#define LLCAP_UTILS

#include "argMapping.hpp"
#include "llvm/Support/raw_ostream.h"
#include <charconv>
#include <optional>

template <class T, class StrT> std::optional<T> tryParse(const StrT &S) {
  T Res;
  auto [_, ec] = std::from_chars(S.data(), S.data() + S.size(), Res);

  if (ec != std::errc()) {
    IF_VERBOSE llvm::errs() << "Warning - invalid numeric value: " << S << '\n';
    return std::nullopt;
  }

  return Res;
}

template <class T> T valOrDefault(std::optional<T> Opt, T Default) {
  return Opt ? *Opt : Default;
}

#endif