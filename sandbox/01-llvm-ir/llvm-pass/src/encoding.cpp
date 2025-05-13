#include "encoding.hpp"
#include "typeAlias.hpp"
#include "llvm/Support/raw_ostream.h"
#include <filesystem>
#include <fstream>

ModuleMappingEncoding::ModuleMappingEncoding(const std::string &maps_directory,
                                             const std::string &name,
                                             const std::string &module_name) {
  Str Path = maps_directory + '/' + name;
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

  m_file << module_name << m_sep;
  // plain m_file just does not work?
  m_failed = !m_file;
}

bool ModuleMappingEncoding::ready() { return !m_failed; }

bool ModuleMappingEncoding::addFunction(const std::string &functionName,
                                        llcap::FunctionId function) {
  if (!ready()) {
    return false;
  }

  m_file << functionName << m_innserSep << function << m_sep;

  if (!m_file) {
    llvm::errs() << "Could not add function " << '\n';
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