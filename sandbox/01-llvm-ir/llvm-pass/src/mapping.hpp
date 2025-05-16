#ifndef LLCPASS_MAPPING
#define LLCPASS_MAPPING

#include "argMapping.hpp"
#include "constants.hpp"
#include "typeids.h"
#include <array>
#include <cassert>
#include <string>
#include <utility>
#include <vector>

class FunctionIDMapper {
  using SizeList = std::vector<std::pair<size_t, LlcapSizeType>>;
  using FunctionData =
      std::tuple<std::string, llcap::FunctionId,
                 std::vector<std::pair<size_t, LlcapSizeType>>>;
  llcap::ModuleId ModuleIntId{0};
  std::string FullModuleId;
  std::string OutFileName;
  std::vector<FunctionData> FunctionIds;
  llcap::FunctionId FunctionId{0};
  static constexpr size_t SHA256_BYTES = 32;

  static std::array<uint8_t, sizeof(llcap::ModuleId)>
  collapseHash(const std::array<uint8_t, SHA256_BYTES> &Data);

public:
  static bool flush(FunctionIDMapper &&Mapper, const std::string &TargetDir);

  FunctionIDMapper(const std::string &ModuleId);
  llcap::FunctionId addFunction(const std::string &FnInfo,
                                ClangMetadataToLLVMArgumentMapping &Mapping);
  [[nodiscard]] const std::string &getFullModuleId() const {
    return FullModuleId;
  }
  [[nodiscard]] const std::string &getModuleMapId() const {
    return OutFileName;
  }
  [[nodiscard]] llcap::ModuleId getModuleMapIntId() const {
    return ModuleIntId;
  }
};

#endif