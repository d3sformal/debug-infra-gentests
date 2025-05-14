#include "argMapping.hpp"
#include "typeAlias.hpp"
#include "utility.hpp"
#include <algorithm>
#include <cassert>
#include <iterator>

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