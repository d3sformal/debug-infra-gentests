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
  ModuleMappingEncoding(const std::string &maps_directory,
                        const std::string &name);

  static bool finish(ModuleMappingEncoding &&self);

  bool ready();
  bool addFunction(const std::string &, llcap::FunctionId);
};

#endif