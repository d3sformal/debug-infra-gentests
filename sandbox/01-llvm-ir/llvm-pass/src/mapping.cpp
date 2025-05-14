#include "mapping.hpp"

#include "constants.hpp"
#include "encoding.hpp"
#include "typeAlias.hpp"
#include <llvm/ADT/StringRef.h>
#include "llvm/Support/SHA256.h"
#include "llvm/Support/raw_ostream.h"
#include <cassert>
#include <iomanip>
#include <sstream>

Arr<u8, sizeof(llcap::ModuleId)>
FunctionIDMapper::collapseHash(const Arr<u8, SHA256_BYTES> &Data) {
  static_assert(SHA256_BYTES % sizeof(llcap::ModuleId) == 0,
                "invalid hash size or id size");
  Arr<u8, sizeof(llcap::ModuleId)> Res = {0};

  // xor every MODULE_ID_BYTE_SIZE bytes with the previous MODULE_ID_BYTE_SIZE
  // btytes
  for (size_t I = 0; I < SHA256_BYTES; I += Res.size()) {
    for (size_t J = 0; J < Res.size(); J += 1) {
      assert(I < Data.size() && "Wrong indicies");
      Res[J] = (Res[J] ^ Data[I + J]);
    }
  }

  return Res;
}

FunctionIDMapper::FunctionIDMapper(const Str &ModuleId)
    : FullModuleId(ModuleId) {
  constexpr size_t BYTE_SIZE = 8;
  llvm::SHA256 Hash;
  Hash.init();
  Hash.update(llvm::StringRef(ModuleId));
  std::stringstream SStream;
  SStream << std::hex << std::setfill('0');

  const auto Collapsed = collapseHash(Hash.result());
  for (auto &&C : Collapsed) {
    ModuleIntId = (ModuleIntId
                   << sizeof(C) * BYTE_SIZE); // shift first to avoid overshift
                                              // in the last iteration
    ModuleIntId += C;

    SStream << std::setw(llcap::BYTE_ENCODING_SIZE)
            << static_cast<unsigned int>(C);
  }
  OutFileName = SStream.str();
}

llcap::FunctionId FunctionIDMapper::addFunction(const Str &FnInfo) {
  auto Inserted = FunctionId++;
  FunctionIds.emplace_back(FnInfo, Inserted);
  return Inserted;
}

bool FunctionIDMapper::flush(FunctionIDMapper &&Mapper, const Str &TargetDir) {
  auto LocalMapper = std::move(Mapper);
  Str Dir = TargetDir.size() > 0 ? TargetDir : "module-maps";
  ModuleMappingEncoding Encoding(Dir, LocalMapper.getModuleMapId(),
                                 LocalMapper.getFullModuleId());

  for (auto &&IdPair : LocalMapper.FunctionIds) {
    if (!Encoding.ready()) {
      llvm::errs() << "Encoding failed\n";
      return false;
    }
    Str &FnName = IdPair.first;
    llcap::FunctionId Id = IdPair.second;

    Encoding.addFunction(FnName, Id);
  }

  if (!Encoding.ready()) {
    llvm::errs() << "Encoding failed @ end\n";
    return false;
  }
  return ModuleMappingEncoding::finish(std::move(Encoding));
}
