#ifndef LLCAP_TYPEMAP
#define LLCAP_TYPEMAP

#include "typeids.h"
#include "utility.hpp"
#include "llvm/ADT/StringRef.h"
#include "llvm/IR/Function.h"
#include <map>
#include <set>
#include <vector>

// this struct hold separator characters used when parsing the index mapping metadata
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

  // returns the size type hint corresponding to the specified LLVM IR argument (by index)
  [[nodiscard]] LlcapSizeType llvmArgNoSizeType(unsigned int LlvmArgNo) const;

public:
  ClangMetadataToLLVMArgumentMapping(llvm::Function &Fn, IdxMappingInfo Seps);

  // uses registered argument indicies to check whether the llvm argument number (index) 
  // matches with registered extension type at the specified metadata key
  [[nodiscard]] bool llvmArgNoMatches(size_t LlvmArgNo,
                                      const llvm::StringRef &MetadataKey) const;

  // reads metadata of the current function and tries to register indicies
  // of the custom types
  // indicies are expected to be encoded under a metedata key, if such key is not
  // present, functions does not contain arguments of the custom type associated with
  // the supplied metadatata key
  bool registerCustomTypeIndicies(const llvm::StringRef &MetadataKey,
                                  LlcapSizeType AssociatedSize);

  // returns pairs of (LLVM arg index) - (size type)
  // where LlcapSizeType::LLSZ_INVALID means that no size can be determined via
  // neither custom type mapping as registered in this object or a primitive type
  [[nodiscard]] std::vector<std::pair<size_t, LlcapSizeType>>
  getArgumentSizeTypes() const;
};

#endif