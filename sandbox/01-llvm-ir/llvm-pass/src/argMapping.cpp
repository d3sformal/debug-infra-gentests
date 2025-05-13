#include "argMapping.hpp"
#include "typeAlias.hpp"
#include <charconv>
#include <system_error>

Set<size_t> getSretArgumentIndicies(const llvm::Function &Fn) {
  Set<size_t> Res;
  size_t Idx = 0;
  for (const auto *It = Fn.arg_begin(); It != Fn.arg_end(); ++It) {
    auto *Arg = It;
    if (Arg->hasAttribute(llvm::Attribute::AttrKind::StructRet)) {
      Res.insert(Idx);
    }
    ++Idx;
  }
  return Res;
}

std::vector<size_t> getSretArgumentShiftVec(const llvm::Function &Fn) {
  auto SretIndicies = getSretArgumentIndicies(Fn);

  std::vector<size_t> Res;
  Res.resize(Fn.arg_size());

  size_t Shift = 0;
  for (size_t I = 0; I < Fn.arg_size(); ++I) {
    if (SretIndicies.find(I) != SretIndicies.end()) {
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

std::vector<size_t> parseCustTypeIndicies(llvm::StringRef MetaValue,
                                          bool IsInstanceMember, char Sep) {
  llvm::SmallVector<llvm::StringRef> Split;
  std::vector<ssize_t> Res;

  MetaValue.split(Split, Sep, -1, false);

  std::transform(Split.begin(), Split.end(), std::back_inserter(Res),
                 [](llvm::StringRef s) {
                   long long Res = -1ll;
                   auto [_, ec] =
                       std::from_chars(s.data(), s.data() + s.size(), Res);

                   if (ec != std::errc()) {
                     llvm::errs()
                         << "Warning - invalid numeric value in metadata: " << s
                         << '\n';
                   }

                   return Res;
                 });

  auto EndRes =
      std::remove_if(Res.begin(), Res.end(), [](ssize_t i) { return i == -1; });

  std::vector<size_t> RealRes;
  RealRes.reserve(Res.size());

  std::transform(Res.begin(), EndRes, std::back_inserter(RealRes),
                 [IsInstanceMember](ssize_t i) {
                   assert(i >= 0);
                   return static_cast<size_t>(i) + (IsInstanceMember ? 1 : 0);
                 });

  return RealRes;
}

Maybe<Vec<size_t>> getCustomTypeIndicies(llvm::StringRef MetadataKey,
                                         const llvm::Function &Fn,
                                         bool IsInstanceMember) {
  if (llvm::MDNode *N = Fn.getMetadata(MetadataKey)) {
    if (N->getNumOperands() == 0) {
      llvm::errs()
          << "Warning - unexpected string metadata node with NO operands!\n";
      return std::nullopt;
    }

    if (llvm::MDString *op =
            llvm::dyn_cast_if_present<llvm::MDString>(N->getOperand(0));
        op != nullptr) {
      return parseCustTypeIndicies(op->getString(), IsInstanceMember,
                                   VSTR_LLVM_CXX_SINGLECHAR_SEP);
    }

    llvm::errs()
        << "Warning - unexpected string metadata node with non-MDString "
           "0th operand!\n";
  } else {
    IF_VERBOSE llvm::errs() << "No meta key " << MetadataKey << " found\n";
  }
  return std::nullopt;
}