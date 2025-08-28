#include "util_types.hpp"
#include "valueSerializers.hpp"
#include <cstdlib>
#include <iostream>
#include <memory>
#include <ostream>

#ifndef VSTR_FUNTRACE_LIB_HPP
#define VSTR_FUNTRACE_LIB_HPP

namespace funTraceLib {

enum class ETraceEvent : u8;

class TraceLogger {
  friend struct ScopeDumper;

public:
  static TraceLogger *get();
  TraceLogger(const char *fileName);
  ~TraceLogger();

private:
  void dumpTraceEvent(u64 fnId, ETraceEvent evt);
  static TraceLogger *s_logger;
  std::unique_ptr<std::ostream> m_out{nullptr};
};

struct ScopeDumper {
  ScopeDumper(const char *fnName, u64 fnId);

  void registerReturn();
  ~ScopeDumper();

private:
  const char *fnName{nullptr};
  const u64 fnId;
  bool returned{false};
};

namespace dump {
template <class T> void dumpValue(const T &value) {
  auto buff = serializers::Serializer<T>::serialize(value);
  std::cout << "BuffDump: " << buff.size() << '\n';
  for (auto &&b : buff) {
    std::cout << std::hex << (u32)b << '-';
  }
  std::cout << std::dec << std::endl;
}

// TODO move outside of header if possible
inline void dumpFnId(u64 fnId) { std::cout << "Fn: " << fnId << '\n'; }

// these are convenience functions that made some steps
// simpler in the past
// they should not be used in a final version because of
// the potential explosion of instantiations (for every
// subset of a combination of fn parameter types)
// TODO remove and only use the above in dumping fragments
template <class T> void dumpValues(const T &value) { dumpValue(value); }

template <class T, class... Ts>
void dumpValues(const T &value, const Ts &...rest) {
  dumpValues(value);
  dumpValues(rest...);
}

template <class... Ts> void dumpValuesWithId(u64 fnId, const Ts &...values) {
  dumpFnId(fnId);
  dumpValues(values...);
}
} // namespace dump
} // namespace funTraceLib

#endif // VSTR_FUNTRACE_LIB_HPP