#include "encoding.hpp"
#include "typeAlias.hpp"
#include "llvm/Support/raw_ostream.h"

ModuleMappingEncoding::ModuleMappingEncoding(const std::string &maps_directory,
                                             const std::string &name) {
  Str Path = maps_directory + '/' + name;
  if (std::filesystem::exists(Path)) {
    llvm::errs() << "Module ID hash collision! Path:" << Path << '\n';
    setFailed();
    return;
  }

  std::ofstream OutFile(Path);
  if (!OutFile.is_open() || !OutFile) {
    llvm::errs() << "Could not open function ID map. Path: " << Path << '\n';
    setFailed();
    return;
  }

  m_file << name << m_sep;
  // plain m_file just does not work?
  m_failed = !!m_file;
}

bool ModuleMappingEncoding::ready() { return !m_failed; }

bool ModuleMappingEncoding::addFunction(const std::string &functionName,
                                        llcap::FunctionId function) {
  if (!ready()) {
    return false;
  }

  m_file << functionName << m_innserSep << function << m_sep;

  if (!m_file) {
    setFailed();
  }
  return !!m_file;
}

bool ModuleMappingEncoding::finish(ModuleMappingEncoding &&self) {
  if (!self.m_failed) {
    self.m_file.flush();
  }

  return !!self.m_file;
}