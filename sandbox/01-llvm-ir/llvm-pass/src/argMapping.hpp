#ifndef LLCAP_TYPEMAP
#define LLCAP_TYPEMAP

#include "typeids.h"
#include "verbosity.hpp"
#include "llvm/ADT/StringRef.h"
#include "llvm/IR/Function.h"
#include <map>
#include <set>
#include <vector>

// returns the LLVM argument indicies of Fn's arguments marked with the sret
// attribute
std::set<size_t> getSretArgumentIndicies(const llvm::Function &Fn);

// computes the Sret shift vector for Fn's arguments
// the output vector is the same size as Fn's argument list
// vec[i] = the shift that mut be applied to an AST arg idx
// to convert it to the LLVM arg idx
std::vector<size_t> getSretArgumentShiftVec(const llvm::Function &Fn);

// parses the metadata that encode the indicies of a custom type
//
// indicies are separated by Sep and are decimal numbers
std::vector<size_t> parseCustTypeIndicies(llvm::StringRef MetaValue,
                                          bool IsInstanceMember, char Sep);

struct IdxMappingInfo {
  char primary;
  char group;
  char argParamPair;
  char custom;
  uint64_t invalidIndexValue;
};

std::optional<std::vector<size_t>>
getCustomTypeIndicies(llvm::StringRef MetadataKey, const llvm::Function &Fn,
                      bool IsInstanceMember, IdxMappingInfo Info);

class ClangMetadataToLLVMArgumentMapping {
  // This map encodes at position ShiftMap[i] what shift should we consider
  // when evaluating i-th argument (i.e. how many additional argumens are
  // there without the this pointer - which can be detected in the AST phase
  // and is accounted for)
  std::vector<size_t> m_shiftMap;
  bool m_instanceMember;
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