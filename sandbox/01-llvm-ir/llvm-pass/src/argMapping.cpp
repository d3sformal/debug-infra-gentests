#include "argMapping.hpp"
#include "../../custom-metadata-pass/ast-meta-add/llvm-metadata.h"
#include "typeAlias.hpp"
#include "typeids.h"
#include "utility.hpp"
#include "llvm/Support/raw_ostream.h"
#include <algorithm>
#include <cassert>
#include <iterator>
#include <type_traits>
#include <utility>
#include <vector>

Set<size_t> getSretArgumentIndicies(const llvm::Function &Fn) {
  Set<size_t> Res;
  size_t Idx = 0;
  for (const auto *It = Fn.arg_begin(); It != Fn.arg_end(); ++It) {
    const auto *Arg = It;
    if (Arg->hasAttribute(llvm::Attribute::AttrKind::StructRet)) {
      Res.insert(Idx);
    }
    ++Idx;
  }
  return Res;
}

Vec<size_t> getSretArgumentShiftVec(const llvm::Function &Fn) {
  auto SretIndicies = getSretArgumentIndicies(Fn);

  Vec<size_t> Res;
  Res.resize(Fn.arg_size());

  size_t Shift = 0;
  for (size_t I = 0; I < Fn.arg_size(); ++I) {
    if (SretIndicies.contains(I)) {
      // we accumulate shift to Shift
      // if an AST index would collide with an sret argument's one, the AST
      // index (A) is translated by A + (current) Shift.

      // Shift is incremented as EACH LLVM sret arg pushes all the remaining
      // ones to the right
      Res[I] = ++Shift;
    } else {
      Res[I] = Shift;
    }
  }
  return Res;
}

Vec<size_t> parseCustTypeIndicies(llvm::StringRef MetaValue,
                                  bool IsInstanceMember, char Sep) {
  llvm::SmallVector<llvm::StringRef> Split;
  Vec<ssize_t> Res;

  MetaValue.split(Split, Sep, -1, false);

  std::ranges::transform(Split, std::back_inserter(Res), [](llvm::StringRef S) {
    return valOrDefault(tryParse<long long>(S), -1LL);
  });

  auto [EndRes, _] =
      std::ranges::remove_if(Res, [](ssize_t I) { return I == -1; });

  Vec<size_t> RealRes;
  assert(std::distance(Res.begin(), EndRes) >= 0);
  RealRes.reserve(static_cast<size_t>(std::distance(Res.begin(), EndRes)));

  std::transform(Res.begin(), EndRes, std::back_inserter(RealRes),
                 [IsInstanceMember](ssize_t I) {
                   assert(I >= 0);
                   return static_cast<size_t>(I) + (IsInstanceMember ? 1 : 0);
                 });

  return RealRes;
}

Maybe<Vec<size_t>> getCustomTypeIndicies(llvm::StringRef MetadataKey,
                                         const llvm::Function &Fn,
                                         bool IsInstanceMember,
                                         IdxMappingInfo Info) {
  if (llvm::MDNode *N = Fn.getMetadata(MetadataKey)) {
    if (N->getNumOperands() == 0) {
      llvm::errs()
          << "Warning - unexpected string metadata node with NO operands!\n";
      return NONE;
    }

    if (auto *Op = llvm::dyn_cast_if_present<llvm::MDString>(N->getOperand(0));
        Op != nullptr) {
      return parseCustTypeIndicies(Op->getString(), IsInstanceMember,
                                   Info.custom);
    }

    llvm::errs()
        << "Warning - unexpected string metadata node with non-MDString "
           "0th operand!\n";
  } else {
    IF_VERBOSE llvm::errs() << "No meta key " << MetadataKey << " found\n";
  }
  return NONE;
}

ClangMetadataToLLVMArgumentMapping::ClangMetadataToLLVMArgumentMapping(
    llvm::Function &Fn, IdxMappingInfo Seps)
    : m_fn(Fn), m_seps(Seps) {
  m_shiftMap = getSretArgumentShiftVec(m_fn);
  m_instanceMember =
      m_fn.getMetadata(llvm::StringRef(VSTR_LLVM_CXX_THISPTR)) != nullptr;
}

bool ClangMetadataToLLVMArgumentMapping::registerCustomTypeIndicies(
    const llvm::StringRef &MetadataKey, LlcapSizeType AssociatedSize) {
  auto CustTypeIdcs =
      getCustomTypeIndicies(MetadataKey, m_fn, m_instanceMember, m_seps);
  IF_DEBUG {
    llvm::errs() << "CustType indicies: ";
    if (CustTypeIdcs) {
      for (auto &&I : *CustTypeIdcs) {
        llvm::errs() << I << " ";
      }
    }
    llvm::errs() << '\n';
  }
  std::optional<std::set<size_t>> CustTypeIdxMap =
      CustTypeIdcs
          ? std::optional(std::set(CustTypeIdcs->begin(), CustTypeIdcs->end()))
          : std::nullopt;
  if (CustTypeIdxMap) {
    m_typeIndicies[MetadataKey] =
        std::make_pair(AssociatedSize, *CustTypeIdxMap);
    return true;
  }
  return false;
}

bool ClangMetadataToLLVMArgumentMapping::llvmArgNoMatches(
    size_t LlvmArgNo, const llvm::StringRef &MetadataKey) const {
  IF_DEBUG {
    llvm::errs()
        << "Checking argument index match for argument with llvm index "
        << LlvmArgNo << '\n'
        << "Custom type indicies " << MetadataKey << ": ";
  }
  if (!m_typeIndicies.contains(MetadataKey)) {
    IF_DEBUG { llvm::errs() << "none\n"; }
    return false;
  }

  const auto &[Sz, CustTypes] = m_typeIndicies.at(MetadataKey);
  IF_DEBUG {
    for (const auto &I : CustTypes) {
      llvm::errs() << I << " ";
    }
    llvm::errs() << "\nCustom type size enum: "
                 << std::underlying_type_t<LlcapSizeType>(Sz) << '\n';

    llvm::errs() << "\nShiftMap: ";

    for (const auto &I : m_shiftMap) {
      llvm::errs() << I << " ";
    }
  }

  auto Res = std::ranges::any_of(CustTypes, [&](size_t AstIndex) {
    return AstIndex + m_shiftMap[AstIndex] == LlvmArgNo;
  });

  return Res;
}

// returns pairs of (LLVM arg index) - (size type)
// where LlcapSizeType::LLSZ_INVALID means that no size can be determined via
// neither custom type mapping as registered in this object or a primitive type
Vec<Pair<size_t, LlcapSizeType>>
ClangMetadataToLLVMArgumentMapping::getArgumentSizeTypes() const {
  Vec<Pair<size_t, LlcapSizeType>> Res;
  Res.reserve(m_fn.arg_size());

  for (unsigned int I = 0; I < m_fn.arg_size(); ++I) {
    Res.emplace_back(I, llvmArgNoSizeType(I));
  }
  return Res;
}

LlcapSizeType ClangMetadataToLLVMArgumentMapping::llvmArgNoSizeType(
    unsigned int LlvmArgNo) const {
  LlcapSizeType Res = LlcapSizeType::LLSZ_INVALID;
  for (auto &&[CustTName, Desc] : m_typeIndicies) {
    auto &&[SizeType, _] = Desc;
    const auto Sz = SizeType;

    if (Sz != LlcapSizeType::LLSZ_INVALID &&
        llvmArgNoMatches(LlvmArgNo, CustTName)) {
      if (Res == LlcapSizeType::LLSZ_INVALID) {
        Res = Sz;
      } else {
        llvm::errs() << "Serious error!!!\nLLVM Arg Number" << LlvmArgNo
                     << " of function " << m_fn.getName()
                     << " found to be associated with > 1 custom size "
                        "types!\nThe current one is: "
                     << CustTName << "\nThis should not happen!";
        // exit here?
      }
    }
  }

  if (Res != LlcapSizeType::LLSZ_INVALID) {
    return Res;
  }

  const auto *Arg = m_fn.getArg(LlvmArgNo);
  const auto *ArgT = Arg->getType();

  if (ArgT->isFloatTy()) {
    return LlcapSizeType::LLSZ_32;
  }

  if (ArgT->isDoubleTy()) {
    return LlcapSizeType::LLSZ_64;
  }

  const Arr<Pair<unsigned int, LlcapSizeType>, 4> IntTypeSizeMap = {
      Pair{8, LlcapSizeType::LLSZ_8},
      {16, LlcapSizeType::LLSZ_16},
      {32, LlcapSizeType::LLSZ_32},
      {64, LlcapSizeType::LLSZ_64}};

  for (auto &&[I, S] : IntTypeSizeMap) {
    if (ArgT->isIntegerTy(I)) {
      return S;
    }
  }

  return LlcapSizeType::LLSZ_INVALID;
}