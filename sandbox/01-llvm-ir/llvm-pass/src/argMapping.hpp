#ifndef LLCAP_TYPEMAP
#define LLCAP_TYPEMAP

#include "typeids.h"
#include "utility.hpp"
#include "llvm/ADT/StringRef.h"
#include "llvm/IR/Function.h"
#include <map>
#include <set>
#include <vector>

struct IdxMappingInfo {
  char primary;
  char group;
  char argParamPair;
  char custom;
  uint64_t invalidIndexValue;

  static std::optional<IdxMappingInfo> parseFromModule(llvm::Module &M);
};

class ClangMetadataToLLVMArgumentMapping {
  std::vector<size_t> m_astArgIdxToLlvmArgIdx;
  std::vector<size_t> m_astArgIdxToLlvmArgLen;
  bool m_instanceMember;
  // tag (e.g. VSTR_LLVM_CXX_DUMP_STDSTRING) -> pairs of (sizeType, indicies of
  // all arguments of this sizeType)
  std::map<llvm::StringRef, std::pair<LlcapSizeType, std::set<size_t>>>
      m_typeIndicies;
  llvm::Function &m_fn;
  IdxMappingInfo m_seps;

  [[nodiscard]] LlcapSizeType llvmArgNoSizeType(unsigned int LlvmArgNo) const;

public:
  ClangMetadataToLLVMArgumentMapping(llvm::Function &Fn, IdxMappingInfo Seps);

  [[nodiscard]] bool llvmArgNoMatches(size_t LlvmArgNo,
                                      const llvm::StringRef &MetadataKey) const;

  bool registerCustomTypeIndicies(const llvm::StringRef &MetadataKey,
                                  LlcapSizeType AssociatedSize);

  [[nodiscard]] std::vector<std::pair<size_t, LlcapSizeType>>
  getArgumentSizeTypes() const;
};

#endif