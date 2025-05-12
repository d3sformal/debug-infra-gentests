#include "mapping.hpp"

#include "constants.hpp"
#include "encoding.hpp"
#include "typeAlias.hpp"
#include "llvm/Support/SHA256.h"
#include "llvm/Support/raw_ostream.h"
#include <array>
#include <cassert>
#include <iomanip>
#include <llvm/ADT/StringRef.h>
#include <sstream>

Arr<u8, sizeof(llcap::ModuleId)>
FunctionIDMapper::collapseHash(const Arr<u8, sha256Bytes> &data) {
  static_assert(sha256Bytes % sizeof(llcap::ModuleId) == 0,
                "invalid hash size or id size");
  Arr<u8, sizeof(llcap::ModuleId)> res = {0};

  // xor every MODULE_ID_BYTE_SIZE bytes with the previous MODULE_ID_BYTE_SIZE
  // btytes
  for (size_t i = 0; i < sha256Bytes; i += res.size()) {
    for (size_t j = 0; j < res.size(); j += 1) {
      assert(i < data.size() && "Wrong indicies");
      res[j] = (res[j] ^ data[i + j]);
    }
  }

  return res;
}

FunctionIDMapper::FunctionIDMapper(const Str &ModuleId)
    : FullModuleId(ModuleId) {
  llvm::SHA256 hash;
  hash.init();
  hash.update(llvm::StringRef(ModuleId));
  std::stringstream SStream;
  SStream << std::hex << std::setfill('0');

  const auto collapsed = collapseHash(hash.result());
  for (auto &&C : collapsed) {
    ModuleIntId =
        (ModuleIntId
         << sizeof(C) *
                8); // shift first to avoid overshift in the last iteration
    ModuleIntId += C;

    SStream << std::setw(llcap::BYTE_ENCODING_SIZE) << (unsigned int)C;
  }
  OutFileName = SStream.str();
}

llcap::FunctionId FunctionIDMapper::addFunction(const Str &FnInfo) {
  auto Inserted = FunctionId++;
  FunctionIds.emplace_back(FnInfo, Inserted);
  return Inserted;
}

bool FunctionIDMapper::flush(FunctionIDMapper &&mapper, const Str &targetDir) {
  Str Dir = targetDir.size() > 0 ? targetDir : "module-maps";
  ModuleMappingEncoding encoding(Dir, mapper.GetModuleMapId(), mapper.GetFullModuleId());

  for (auto &&IdPair : mapper.FunctionIds) {
    if (!encoding.ready()) {
      llvm::errs() << "Encoding failed\n";
      return false;
    }
    Str &fn_name = IdPair.first;
    llcap::FunctionId Id = IdPair.second;

    encoding.addFunction(fn_name, Id);
  }

  if (!encoding.ready()) {
    llvm::errs() << "Encoding failed @ end\n";
    return false;
  }
  return ModuleMappingEncoding::finish(std::move(encoding));
}