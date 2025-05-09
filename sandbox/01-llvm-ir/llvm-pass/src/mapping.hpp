#ifndef LLCPASS_MAPPING
#define LLCPASS_MAPPING

#include "constants.hpp"
#include <array>
#include <cassert>
#include <llvm/ADT/StringRef.h>
#include <string>
#include <vector>

class FunctionIDMapper {
  llcap::ModuleId ModuleIntId{0};
  std::string FullModuleId;
  std::string OutFileName;
  std::vector<std::pair<std::string, llcap::FunctionId>> FunctionIds;
  llcap::FunctionId FunctionId{0};
  static constexpr size_t sha256Bytes = 32;

  static std::array<uint8_t, sizeof(llcap::ModuleId)>
  collapseHash(const std::array<uint8_t, sha256Bytes> &data);

public:
  static bool flush(FunctionIDMapper &&mapper, const std::string &targetDir);

  FunctionIDMapper(const std::string &ModuleId);
  llcap::FunctionId addFunction(const std::string &FnInfo);
  const std::string &GetModuleMapId() const { return OutFileName; }
  llcap::ModuleId GetModuleMapIntId() const { return ModuleIntId; }
};

#endif