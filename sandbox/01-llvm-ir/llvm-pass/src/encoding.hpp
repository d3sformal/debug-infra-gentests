#ifndef LLCPASS_ENCODING
#define LLCPASS_ENCODING

#include "constants.hpp"
#include <fstream>

class ModuleMappingEncoding {
  char m_sep{'\n'};
  char m_innserSep{'\0'};
  std::ofstream m_file;
  bool m_failed{true};
  void setFailed() { m_failed = true; };

public:
  ModuleMappingEncoding(const std::string &MapsDirectory,
                        const std::string &FileName,
                        const std::string &ModuleName);

  static bool finish(ModuleMappingEncoding &&Self);

  bool ready() const { return !m_failed; }

  bool encodeFunction(const std::string &, llcap::FunctionId);
};

#endif