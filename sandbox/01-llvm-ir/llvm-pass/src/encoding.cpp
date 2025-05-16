#include "encoding.hpp"
#include "typeAlias.hpp"
#include "llvm/Support/raw_ostream.h"
#include <filesystem>
#include <fstream>
#include <utility>

ModuleMappingEncoding::ModuleMappingEncoding(const Str &MapsDirectory,
                                             const Str &Name,
                                             const Str &ModuleName) {
  Str Path = MapsDirectory + '/' + Name;
  if (std::filesystem::exists(Path)) {
    llvm::errs() << "Module ID hash collision! Path:" << Path << '\n';
    setFailed();
    return;
  }

  m_file = std::ofstream(Path);
  if (!m_file.is_open() || !m_file) {
    llvm::errs() << "Could not open function ID map. Path: " << Path << '\n';
    setFailed();
    return;
  }

  m_file << ModuleName << m_sep;
  // plain m_file just does not work?
  m_failed = !m_file;
}

bool ModuleMappingEncoding::finish(ModuleMappingEncoding &&Self) {
  auto Local = std::move(Self);
  if (!Local.m_failed) {
    Local.m_file.flush();
  }

  return !!Local.m_file;
}