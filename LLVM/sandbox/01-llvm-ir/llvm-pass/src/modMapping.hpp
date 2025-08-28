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

// generates function IDs and maps the ID to the user-readable identifier
// the instances are indended to be used on a per-module basis (a module
// corresponds to at most one instance of FunctionIDMapper)
class FunctionIDMapper {
  using SizeList = std::vector<std::pair<size_t, LlcapSizeType>>;
  using FunctionData =
      std::tuple<std::string, llcap::FunctionId,
                 std::vector<std::pair<size_t, LlcapSizeType>>>;
  llcap::ModuleId m_moduleIntId{0};
  std::string m_fullModuleId;
  std::string m_outFileName;
  std::vector<FunctionData> m_functionIds;
  // auto-incrementing ID
  llcap::FunctionId m_functionIdGenerator{0};

public:
  static constexpr size_t SHA256_BYTES = 32;
  using FullHashT = std::array<uint8_t, SHA256_BYTES>;
  using ShortHashT = std::array<uint8_t, sizeof(llcap::ModuleId)>;

  // flush the mapper, writing recorded mapping into the TargetDir,
  // discarding the mapper in the process
  // creates a file correspodning to the module's shortened ID
  [[nodiscard]] static bool flush(FunctionIDMapper &&Mapper,
                                  const std::string &TargetDir);

  FunctionIDMapper(const std::string &ModuleId);
  llcap::FunctionId addFunction(const std::string &FnInfo,
                                ClangMetadataToLLVMArgumentMapping &Mapping);
  [[nodiscard]] const std::string &getFullModuleId() const {
    return m_fullModuleId;
  }
  [[nodiscard]] const std::string &getModuleMapId() const {
    return m_outFileName;
  }
  [[nodiscard]] llcap::ModuleId getModuleMapIntId() const {
    return m_moduleIntId;
  }
};

#endif