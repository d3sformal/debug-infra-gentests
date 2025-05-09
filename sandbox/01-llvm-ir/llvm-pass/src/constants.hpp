#ifndef LLVM_PASS_CONSTANTS
#define LLVM_PASS_CONSTANTS
#include <cstdint>

namespace llcap {
using FunctionId = uint32_t;
using ModuleId = uint32_t;
constexpr std::size_t MODULE_ID_BYTE_SIZE = 4;
constexpr std::size_t BYTE_ENCODING_SIZE = 2;
constexpr std::size_t MODULE_ID_CHAR_SIZE =
    MODULE_ID_BYTE_SIZE * BYTE_ENCODING_SIZE;
} // namespace llcap

#endif