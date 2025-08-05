#include "argMapping.hpp"
#include "../../custom-metadata-pass/ast-meta-add/llvm-metadata.h"
#include "typeAlias.hpp"
#include "typeids.h"
#include "utility.hpp"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/raw_ostream.h"
#include <algorithm>
#include <cassert>
#include <cstddef>
#include <iterator>
#include <ranges>
#include <type_traits>
#include <utility>
#include <vector>

namespace {
// parsing the argument mapping list (0-1#1-1#2-1#4294967295-0), see
// initParseArgMapping
bool parseArgMappingList(size_t LlArgCnt, size_t AstArgCnt, llvm::StringRef Str,
                         std::vector<size_t> &OutArgStarts, IdxMappingInfo Seps,
                         std::vector<size_t> &OutArgSizes) {
  // it does not make sense to parse more arguments than present in LLVM IR
  size_t Count = std::min(LlArgCnt, AstArgCnt);

  llvm::SmallVector<llvm::StringRef> Split;
  Str.split(Split, Seps.group, -1, true);

  if (Split.size() < Count) {
    llvm::errs() << "Malformed metadata - arg mapping list size ( " << Count
                 << " )\n";
    return false;
  }

  using ParseResT = Maybe<Pair<size_t, size_t>>;

  auto Parsed =
      std::ranges::take_view(Split, Count) |
      std::ranges::views::transform([Seps](llvm::StringRef S) -> ParseResT {
        llvm::SmallVector<llvm::StringRef> PairSplit;
        S.split(PairSplit, Seps.argParamPair);

        if (PairSplit.size() != 2) {
          llvm::errs() << "Malformed metadata - mapping pair\n";
          return NONE;
        }

        auto LlIdxStart = tryParse<size_t>(PairSplit[0]);
        auto LlArgSpan = tryParse<size_t>(PairSplit[1]);

        if (!LlIdxStart || !LlArgSpan) {
          llvm::errs() << "Malformed metadata - pair values";
          return NONE;
        }

        return std::make_pair(*LlIdxStart, *LlArgSpan);
      });

  auto Failed = std::ranges::any_of(
      Parsed, [](const ParseResT &V) { return !V.has_value(); });
  if (Failed) {
    return false;
  }

  auto Filtered =
      std::ranges::views::transform(Parsed,
                                    [](const ParseResT &V) { return *V; }) |
      std::ranges::views::filter([Seps](const Pair<size_t, size_t> &V) {
        return V.first != Seps.invalidIndexValue;
      });

  for (auto &&[IdxStart, ArgSpan] : Filtered) {
    OutArgStarts.push_back(IdxStart);
    OutArgSizes.push_back(ArgSpan);
  }

  return true;
}

// parsing metadata stirng in the form
// 3 4 0-1#1-1#2-1#4294967295-0
// in the form
// LLVM-args AST-args LIST(size = AST-args)
// where LIST maps AST index to a pair (LLVM start index, LLVM arg span)
// Seps (in this case) should be { primary = ' ', group = '#', pair = '-',
// invalidIdx = 4294967295 }
bool parseArgMapping(llvm::MDNode *ArgMappingMetadata,
                     std::vector<size_t> &OutArgStarts, IdxMappingInfo Seps,
                     std::vector<size_t> &OutArgSizes) {
  if (ArgMappingMetadata == nullptr) {
    return false;
  }

  IF_DEBUG { ArgMappingMetadata->dumpTree(); }

  // "0 0 " is the minimal valid metadata string
  constexpr auto MIN_MTV_SIZE = sizeof("0 0 ") - 1;
  llvm::StringRef MetaValue;

  if (auto *Op = llvm::dyn_cast_if_present<llvm::MDString>(
          ArgMappingMetadata->getOperand(0));
      Op != nullptr) {
    MetaValue = Op->getString();
    if (MetaValue.size() < MIN_MTV_SIZE) {
      llvm::errs() << "Malformed metadata - size\n";
      return false;
    }
  } else {
    llvm::errs() << "Missing string value\n";
    return false;
  }

  llvm::SmallVector<llvm::StringRef> Split;
  MetaValue.split(Split, Seps.primary, -1, true);

  if (Split.size() != 3) {
    llvm::errs() << "Malformed metadata - primary split\n";
    return false;
  }

  Vec<size_t> ResStartLLIdxMap;
  Vec<size_t> ResLLArgSize;
  Arr<size_t, 2> ArgCounts;

  assert(ArgCounts.size() < Split.size());

  auto Parsed = std::ranges::take_view(Split, ArgCounts.size()) |
                std::ranges::views::transform(
                    [](llvm::StringRef S) { return tryParse<size_t>(S); });

  size_t Idx = 0;
  // TODO if switched to C++23, use zip+iota or enumerate range
  for (auto MbParsed : Parsed) {
    if (!MbParsed) {
      llvm::errs() << "Malformed metadata - primary split at " << Idx << '\n';
      return false;
    }
    // ranges::take above should ensure correctness, .at just to be sure
    ArgCounts.at(Idx) = *MbParsed;
    Idx++;
  }

  auto [LlvmArgCount, AstArgCount] = ArgCounts;

  if (AstArgCount == 0) {
    return true;
  }
  return parseArgMappingList(LlvmArgCount, AstArgCount, Split[2], OutArgStarts,
                             Seps, OutArgSizes);
}

// parses the metadata that encode the indicies of a custom type
//
// indicies are separated by Sep and are decimal numbers
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

} // namespace

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
  IF_DEBUG { llvm::errs() << Fn.getName() << ": \n"; }
  // metadata under this key are inserted in the patched clang
  // (the literal is part of the patch)
  assert(parseArgMapping(Fn.getMetadata("LLCAP-CLANG-LLVM-MAP-DATA"),
                         m_astArgIdxToLlvmArgIdx, m_seps,
                         m_astArgIdxToLlvmArgLen));

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

    llvm::errs() << "Starts: ";

    for (const auto &I : m_astArgIdxToLlvmArgIdx) {
      llvm::errs() << I << " ";
    }

    llvm::errs() << "\nSizes : ";

    for (const auto &I : m_astArgIdxToLlvmArgLen) {
      llvm::errs() << I << " ";
    }
    llvm::errs() << '\n';
  }

  auto Res = std::ranges::any_of(CustTypes, [&](size_t AstIndex) {
    return m_astArgIdxToLlvmArgIdx.at(AstIndex) == LlvmArgNo;
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