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
constexpr std::size_t BYTE_BITS = 8;
constexpr std::size_t MODID_BITSIZE = sizeof(llcap::ModuleId) * BYTE_BITS;
constexpr std::size_t FUNID_BITSIZE = sizeof(llcap::FunctionId) * BYTE_BITS;
} // namespace llcap

#endif