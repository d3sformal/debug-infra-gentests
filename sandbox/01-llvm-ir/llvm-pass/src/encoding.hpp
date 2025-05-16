#ifndef LLCPASS_ENCODING
#define LLCPASS_ENCODING

#include "constants.hpp"
#include "typeids.h"
#include "llvm/Support/raw_ostream.h"
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

  bool encodeFunction(const std::string &FunctionName,
                      llcap::FunctionId Function,
                      std::ranges::input_range auto &&ArgSizes) {
    if (!ready()) {
      return false;
    }

    m_file << FunctionName << m_innserSep << Function << m_innserSep;
    m_file << ArgSizes.size(); // if zero, nothing follows (only the m_sep)

    // otherwise a list of "size" values follows
    for (const LlcapSizeType &Type : ArgSizes) {
      m_file << m_innserSep << std::underlying_type_t<LlcapSizeType>(Type);
    }

    m_file << m_sep;

    if (!m_file) {
      llvm::errs() << "Could not add function " << '\n';
      setFailed();
    }
    return !!m_file;
  }
};

#endif