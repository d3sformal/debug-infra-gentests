#ifndef LLCAP_TYPEMAP
#define LLCAP_TYPEMAP

#include "../../custom-metadata-pass/ast-meta-add/llvm-metadata.h"
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

std::optional<std::vector<size_t>>
getCustomTypeIndicies(llvm::StringRef MetadataKey, const llvm::Function &Fn,
                      bool IsInstanceMember);

class ClangMetadataToLLVMArgumentMapping {
  // This map encodes at position ShiftMap[i] what shift should we consider
  // when evaluating i-th argument (i.e. how many additional argumens are
  // there without the this pointer - which can be detected in the AST phase
  // and is accounted for)
  std::vector<size_t> m_shiftMap;
  bool m_instanceMember;
  std::map<llvm::StringRef, std::set<size_t>> m_typeIndicies;
  llvm::Function &m_fn;

public:
  ClangMetadataToLLVMArgumentMapping(llvm::Function &Fn) : m_fn(Fn) {
    m_shiftMap = getSretArgumentShiftVec(m_fn);
    m_instanceMember =
        m_fn.getMetadata(llvm::StringRef(VSTR_LLVM_CXX_THISPTR)) != nullptr;
  }
  bool llvmArgNoMatches(size_t LlvmArgNo,
                        const llvm::StringRef &MetadataKey) const {
    IF_VERBOSE {
      llvm::errs()
          << "Checking argument index match for argument with llvm index "
          << LlvmArgNo << '\n'
          << "Custom type indicies " << MetadataKey << ": ";
    }
    if (m_typeIndicies.find(MetadataKey) == m_typeIndicies.end()) {
      IF_VERBOSE { llvm::errs() << "none\n"; }
      return false;
    }

    const auto &CustTypes = m_typeIndicies.at(MetadataKey);
    IF_VERBOSE {
      for (auto &i : CustTypes) {
        llvm::errs() << i << " ";
      }
      llvm::errs() << "\nShiftMap: ";

      for (auto &i : m_shiftMap) {
        llvm::errs() << i << " ";
      }
    }

    auto Res =
        std::any_of(CustTypes.begin(), CustTypes.end(), [&](size_t AstIndex) {
          return AstIndex + m_shiftMap[AstIndex] == LlvmArgNo;
        });

    IF_VERBOSE llvm::errs() << "\nResult: " << Res << '\n';
    return Res;
  }

  bool registerCustomTypeIndicies(const llvm::StringRef &MetadataKey) {
    auto CustTypeIdcs =
        getCustomTypeIndicies(MetadataKey, m_fn, m_instanceMember);
    IF_VERBOSE {
      llvm::errs() << "CustType indicies: ";
      if (CustTypeIdcs) {
        for (auto &&i : *CustTypeIdcs) {
          llvm::errs() << i << " ";
        }
      }
      llvm::errs() << '\n';
    }
    std::optional<std::set<size_t>> CustTypeIdxMap =
        CustTypeIdcs ? std::optional(
                           std::set(CustTypeIdcs->begin(), CustTypeIdcs->end()))
                     : std::nullopt;
    if (CustTypeIdxMap) {
      m_typeIndicies[MetadataKey] = *CustTypeIdxMap;
      return true;
    }
    return false;
  }
};

#endif