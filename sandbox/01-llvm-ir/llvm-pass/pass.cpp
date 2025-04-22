#include "llvm/Pass.h"
#include "../custom-metadata-pass/ast-meta-add/llvm-metadata.h"
#include "llvm/ADT/APInt.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/IR/Function.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/Metadata.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/PassManager.h"
#include "llvm/Passes/PassBuilder.h"
#include "llvm/Passes/PassPlugin.h"
#include "llvm/Support/Casting.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/SHA256.h"
#include "llvm/Support/raw_ostream.h"
#include <fstream>
#include <llvm/ADT/StringRef.h>
#include <llvm/Analysis/TargetLibraryInfo.h>
#include <llvm/Demangle/Demangle.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/GlobalVariable.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Type.h>
#include <llvm/Passes/OptimizationLevel.h>

#include <algorithm>
#include <bits/stdc++.h>
#include <cstddef>
#include <cstdlib>
#include <iterator>
#include <optional>
#include <sstream>
#include <string>

using namespace llvm;

const char *UNSIGNED_ATTR_KIND = "VSTR-param-attr-unsigned";
static uint64_t FunctionId;

namespace {

// -mllvm -llcap-filter-by-mangled
cl::opt<bool>
    MangleFilter("llcap-filter-by-mangled",
                 cl::desc("Filter functions by their mangled names instead of "
                          "their position within the AST"));
// -mllvm -llcap-verbose
cl::opt<bool> Verbose("llcap-verbose", cl::desc("Verbose output"));
// -mllvm -llcap-mapdir
cl::opt<std::string>
    MapFilesDirectory("llcap-mapdir",
                      cl::desc("Directory for function ID maps"));

#define IF_VERBOSE if (Verbose.getValue())
template <class T> using Vec = std::vector<T>;
template <class T> using Maybe = std::optional<T>;
template <class T> using Set = std::set<T>;

// there is no way to tell built-ins from user functions in the IR,
// we can only query external linkage and whether a function is a "declaration"
// this function examines the mangled name of a function and tells (nonportably)
// which function is and is not in the std:: namespace (_Z*St/i/c/a/o/e/s) or
// has a reserved name (two underscores)
bool isStdFnDanger(const StringRef mangled) {
  return mangled.starts_with("_ZNSt") || mangled.starts_with("_ZZNSt") ||
         mangled.starts_with("_ZSt") || mangled.starts_with("_ZNSo") ||
         mangled.starts_with("_ZNSi") || mangled.starts_with("_ZNSe") ||
         mangled.starts_with("_ZNSc") || mangled.starts_with("_ZNSs") ||
         mangled.starts_with("_ZNSa") || mangled.starts_with("__");
}

GlobalVariable *createGlobalStr(Module &M, StringRef Val, StringRef Id) {
  LLVMContext &Ctx = M.getContext();

  Constant *StrConst = ConstantDataArray::getString(Ctx, Val, true);
  GlobalVariable *GV = new GlobalVariable(
      M, StrConst->getType(), true, GlobalValue::PrivateLinkage, StrConst, Id);

  GV->setUnnamedAddr(GlobalValue::UnnamedAddr::Global);
  GV->setAlignment(Align(1));
  return GV;
}

void insertFnEntryLog(IRBuilder<> &Builder, Module &M, GlobalVariable *Message,
                      GlobalVariable *ModuleId) {
  auto *IRStrPtr = Builder.CreateBitCast(Message, Builder.getPtrTy());
  auto *IRModStrPtr = Builder.CreateBitCast(ModuleId, Builder.getPtrTy());
  auto *FnIdConstant =
      ConstantInt::get(M.getContext(), APInt(32, FunctionId++));
  auto Callee = M.getOrInsertFunction(
      "hook_start", FunctionType::get(Type::getVoidTy(M.getContext()),
                                      {Builder.getPtrTy(), Builder.getPtrTy(),
                                       FnIdConstant->getType()},
                                      false));
  Builder.CreateCall(Callee, {IRStrPtr, IRModStrPtr, FnIdConstant});
}
// terminology:
// LLVM Argument Index = 0-based index of an argument as seen directly in the
// LLVM IR

//  AST Argument Index  = 0-based idx as seen in the Clang AST

// key differences accounted for: this pointer & sret arguments (returning a
// struct in a register)

// returns the LLVM argument indicies of Fn's arguments marked with the sret
// attribute
Set<size_t> getSretArgumentIndicies(const Function &Fn) {
  Set<size_t> Res;
  size_t Idx = 0;
  for (auto It = Fn.arg_begin(); It != Fn.arg_end(); ++It) {
    auto *Arg = It;
    if (Arg->hasAttribute(Attribute::AttrKind::StructRet)) {
      Res.insert(Idx);
    }
    ++Idx;
  }
  return Res;
}

// computes the Sret shift vector for Fn's arguments
// the output vector is the same size as Fn's argument list
// vec[i] = the shift that mut be applied to an AST arg idx
// to convert it to the LLVM arg idx
Vec<size_t> getSretArgumentShiftVec(const Function &Fn) {
  auto SretIndicies = getSretArgumentIndicies(Fn);

  Vec<size_t> Res;
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

// parses the metadata that encode the indicies of a custom type
//
// indicies are separated by VSTR_LLVM_CXX_SINGLECHAR_SEP and are decimal
// numbers
Vec<size_t> parseCustTypeIndicies(StringRef MetaValue, bool IsInstanceMember) {
  llvm::SmallVector<StringRef> Split;
  Vec<ssize_t> Res;

  MetaValue.split(Split, VSTR_LLVM_CXX_SINGLECHAR_SEP, -1, false);

  std::transform(
      Split.begin(), Split.end(), std::back_inserter(Res), [](StringRef s) {
        try {
          return std::stoll(s.str());
        } catch (...) {
          errs() << "Warning - invalid numeric value in metadata: " << s
                 << '\n';
          return -1ll;
        }
      });

  auto EndRes =
      std::remove_if(Res.begin(), Res.end(), [](ssize_t i) { return i == -1; });

  Vec<size_t> RealRes;
  RealRes.reserve(Res.size());

  std::transform(Res.begin(), EndRes, std::back_inserter(RealRes),
                 [IsInstanceMember](ssize_t i) {
                   assert(i >= 0);
                   return static_cast<size_t>(i) + (IsInstanceMember ? 1 : 0);
                 });

  return RealRes;
}

// TODO: generalize for custom types - "Custom" in this name means "std::string"
Maybe<Vec<size_t>> getCustomTypeIndicies(StringRef MetadataKey,
                                         const Function &Fn,
                                         bool IsInstanceMember) {
  if (MDNode *N = Fn.getMetadata(MetadataKey)) {
    if (N->getNumOperands() == 0) {
      errs() << "Warning - unexpected string metadata node with NO operands!\n";
      return std::nullopt;
    }

    if (MDString *op = dyn_cast_if_present<MDString>(N->getOperand(0));
        op != nullptr) {
      return parseCustTypeIndicies(op->getString(), IsInstanceMember);
    } else {
      errs() << "Warning - unexpected string metadata node with non-MDString "
                "0th operand!\n";
    }
  } else {
    IF_VERBOSE errs() << "No meta key " << MetadataKey << " found\n";
  }
  return std::nullopt;
}

// this is very dumb, but oh well...
// checks if this LLVM argNo matches any "Custom type" AST argument index
bool ArgNumbersMatch(size_t LlvmNumber, const Vec<size_t> &ShiftMap,
                     const Set<size_t> &CustTypes) {
  IF_VERBOSE {
    llvm::errs()
        << "Checking argument index match for argument with llvm index "
        << LlvmNumber << '\n'
        << "Custom type indicies: ";
    for (auto &i : CustTypes) {
      llvm::errs() << i << " ";
    }
    llvm::errs() << "\nShiftMap: ";

    for (auto &i : ShiftMap) {
      llvm::errs() << i << " ";
    }
  }

  auto Res =
      std::any_of(CustTypes.begin(), CustTypes.end(), [&](size_t AstIndex) {
        return AstIndex + ShiftMap[AstIndex] == LlvmNumber;
      });

  IF_VERBOSE llvm::errs() << "\nResult: " << Res << '\n';
  return Res;
}

void callCaptureHook(IRBuilder<> &Builder, Module &M, Argument *Arg,
                     const Vec<size_t> &ShiftMap,
                     const Maybe<Set<size_t>> &CustTypes,
                     const Maybe<Set<size_t>> &UnsignedMap) {
  auto ArgNum = Arg->getArgNo();
  auto &Ctx = M.getContext();
  auto IsAttrUnsgined =
      UnsignedMap && ArgNumbersMatch(ArgNum, ShiftMap, *UnsignedMap);
  auto ArgT = Arg->getType();
  auto CallByte = M.getOrInsertFunction(
      "hook_char",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getInt8Ty(Ctx)}, false));
  auto CallUnsByte = M.getOrInsertFunction(
      "hook_uchar",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getInt8Ty(Ctx)}, false));
  auto CallShort = M.getOrInsertFunction(
      "hook_short",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getInt8Ty(Ctx)}, false));
  auto CallUnsShort = M.getOrInsertFunction(
      "hook_ushort",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getInt8Ty(Ctx)}, false));
  auto CallInt32 = M.getOrInsertFunction(
      "hook_int32",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getInt32Ty(Ctx)}, false));
  auto CallUnsInt32 = M.getOrInsertFunction(
      "hook_uint32",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getInt32Ty(Ctx)}, false));
  auto CallInt64 = M.getOrInsertFunction(
      "hook_int64",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getInt64Ty(Ctx)}, false));
  auto CallUnsInt64 = M.getOrInsertFunction(
      "hook_uint64",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getInt64Ty(Ctx)}, false));
  auto CallFloat = M.getOrInsertFunction(
      "hook_float",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getFloatTy(Ctx)}, false));
  auto CallDouble = M.getOrInsertFunction(
      "hook_double",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getDoubleTy(Ctx)}, false));

  if (ArgT->isIntegerTy(8)) {
    Builder.CreateCall(IsAttrUnsgined ? CallUnsByte : CallByte, {Arg});
  } else if (ArgT->isIntegerTy(16)) {
    Builder.CreateCall(IsAttrUnsgined ? CallUnsShort : CallShort, {Arg});
  } else if (ArgT->isIntegerTy(32)) {
    Builder.CreateCall(IsAttrUnsgined ? CallUnsInt32 : CallInt32, {Arg});
  } else if (ArgT->isIntegerTy(64)) {
    Builder.CreateCall(IsAttrUnsgined ? CallUnsInt64 : CallInt64, {Arg});
  } else if (ArgT->isFloatTy()) {
    Builder.CreateCall(CallFloat, {Arg});
  } else if (ArgT->isDoubleTy()) {
    Builder.CreateCall(CallDouble, {Arg});
  } else if (CustTypes && ArgNumbersMatch(ArgNum, ShiftMap, *CustTypes)) {
    auto CallCxxString = M.getOrInsertFunction(
        "vstr_extra_cxx__string",
        FunctionType::get(Type::getVoidTy(Ctx), {Arg->getType()}, false));
    Builder.CreateCall(CallCxxString, {Arg});
  } else {
    IF_VERBOSE errs() << "Skipping Argument\n";
    IF_VERBOSE errs() << "State of CustTypes: " << CustTypes.has_value()
                      << '\n';
    IF_VERBOSE Arg->dump();
  }
}

class FunctionIDMapper {
  std::string ModuleId;
  std::string OutFileName;
  std::vector<std::pair<std::string, uint64_t>> FunctionIds;
  uint64_t FunctionId{0};

public:
  FunctionIDMapper(const std::string &ModuleId) : ModuleId(ModuleId) {
    SHA256 hash;
    hash.init();
    hash.update(StringRef(ModuleId));
    std::stringstream SStream;
    SStream << std::hex << std::setfill('0');
    for (auto &&C : hash.result()) {
      SStream << std::setw(2) << (unsigned int)C;
    }
    OutFileName = SStream.str();
  }

  void addFunction(const std::string &FnInfo) {
    FunctionIds.emplace_back(FnInfo, FunctionId++);
  }

  const std::string &GetModuleMapId() { return OutFileName; }

  static bool flush(FunctionIDMapper &&mapper) {
    auto OptVal = MapFilesDirectory.getValue();
    std::string Dir = OptVal.size() > 0 ? OptVal : "module-maps";
    std::string Path = Dir + "/" + mapper.OutFileName;
    std::ofstream OutFile(Path);
    if (!OutFile.is_open() || !OutFile) {
      errs() << "Could not open function ID map. Path: " << Path << '\n';
      return false;
    }

    OutFile << mapper.ModuleId << '\n';
    for (auto &&IdPair : mapper.FunctionIds) {
      auto &&Info = IdPair.first;
      uint64_t Id = IdPair.second;

      OutFile << Info << '\0' << Id << '\n';
      if (!OutFile) {
        break;
      }
    }

    if (!OutFile) {
      errs() << "Error writing function ID map. Path: " << Path << '\n';
      return false;
    }
    return true;
  }
};

struct InsertFunctionCallPass : public PassInfoMixin<InsertFunctionCallPass> {

  PreservedAnalyses run(Module &M, ModuleAnalysisManager &AM) {
    IF_VERBOSE errs() << "Running pass on module!\n";
    FunctionIDMapper IDMap(M.getModuleIdentifier());
    auto *IRGlobalModuleMapId =
        createGlobalStr(M, IDMap.GetModuleMapId(), "module-map-id");

    // TODO remove
    if (auto *Meta = M.getNamedMetadata("LLCAP-CLANG-LLVM-MAP-PRSGD")) {
      Meta->dump();
    } else {
      IF_VERBOSE llvm::errs() << "Module meta no parse guide";
    }
    if (auto *Meta = M.getNamedMetadata("LLCAP-CLANG-LLVM-MAP-INVLD_IDX")) {
      Meta->dump();
    } else {
      IF_VERBOSE llvm::errs() << "Module meta no invalid index value";
    }

    for (auto &F : M) {
      // Skip library functions
      auto Name = F.getFunction().getName();
      auto DemangledName = llvm::demangle(Name);

      if (MDNode *N = F.getMetadata("LLCAP-CLANG-LLVM-MAP-DATA")) {
        outs() << DemangledName << ": \n";
        N->dumpTree();
      }

      if (MangleFilter.getValue() && isStdFnDanger(Name)) {
        continue;
      } else if (!MangleFilter.getValue()) {
        IF_VERBOSE errs() << "Metadata of function " << DemangledName << '\n';
        if (MDNode *N = F.getMetadata(VSTR_LLVM_NON_SYSTEMHEADER_FN_KEY)) {
          if (N->getNumOperands() == 0) {
            IF_VERBOSE errs() << "Warning! Unexpected metadata node with no "
                                 "operands! Function: "
                              << Name << ' ' << DemangledName << '\n';
          }

          if (MDString *op = dyn_cast_if_present<MDString>(N->getOperand(0));
              op == nullptr) {
            IF_VERBOSE errs()
                << "Invalid metadata for node in function: " << Name << ' '
                << DemangledName << "\nNode:\n";
            N->dumpTree();
          }
        } else {
          continue;
        }
      }

      auto *IRGlobalDemangledName =
          createGlobalStr(M, DemangledName, F.getName().str() + "string");

      BasicBlock &EntryBB = F.getEntryBlock();
      IRBuilder<> Builder(&EntryBB.front());
      insertFnEntryLog(Builder, M, IRGlobalDemangledName, IRGlobalModuleMapId);

      Set<llvm::Type::TypeID> AllowedTypes;
      {
        using llvm::Type;
        for (auto &&t : {Type::FloatTyID, Type::IntegerTyID, Type::DoubleTyID,
                         Type::PointerTyID}) {
          AllowedTypes.insert(t);
        }
      }

      bool viable = !F.arg_empty();
      if (viable) {
        for (auto Arg = F.arg_begin(); Arg != F.arg_end(); ++Arg) {
          if (AllowedTypes.find(Arg->getType()->getTypeID()) ==
              AllowedTypes.end()) {
            viable = false;
            break;
          }
        }
      }
      // TODO handle viability
      IDMap.addFunction(DemangledName);

      bool InstanceMember = F.getMetadata(VSTR_LLVM_CXX_THISPTR) != nullptr;

      // sret arguments are arguments that serve as a return value ?inside of a
      // register? in some cases when the function *returns a structure by
      // value* e.g. std::string foo(void);

      // theoretically, we can use F.returnDoesNotAlias() (noalias) as a
      // heuristic for structReturn (sret in IR) arguments but that would need a
      // bit more research (for all cases I've seen, the noalias attribute was
      // set for the an sret parameter - makes sense, though I do not know if
      // there are any contradictions to this)

      // This map encodes at position ShiftMap[i] what shift should we consider
      // when evaluating i-th argument (i.e. how many additional argumens are
      // there without the this pointer - which can be detected in the AST phase
      // and is accounted for)
      const auto ShiftMap = getSretArgumentShiftVec(F);

      auto CustTypeIdcs = getCustomTypeIndicies(VSTR_LLVM_CXX_DUMP_STDSTRING, F,
                                                InstanceMember);
      IF_VERBOSE {
        llvm::errs() << "CustType indicies: ";
        if (CustTypeIdcs) {
          for (auto &&i : *CustTypeIdcs) {
            llvm::errs() << i << " ";
          }
        }
        llvm::errs() << '\n';
      }
      auto CustTypeIdxMap = CustTypeIdcs
                                ? std::optional(std::set(CustTypeIdcs->begin(),
                                                         CustTypeIdcs->end()))
                                : std::nullopt;

      auto CustUnsignedIdcs =
          getCustomTypeIndicies(VSTR_LLVM_UNSIGNED_IDCS, F, InstanceMember);
      IF_VERBOSE {
        llvm::errs() << "Unsigned indicies: ";
        if (CustUnsignedIdcs) {
          for (auto &&i : *CustUnsignedIdcs) {
            llvm::errs() << i << " ";
          }
        }
        llvm::errs() << '\n';
      }
      auto UnsignedMap = CustUnsignedIdcs
                             ? std::optional(std::set(CustUnsignedIdcs->begin(),
                                                      CustUnsignedIdcs->end()))
                             : std::nullopt;

      for (auto Arg = F.arg_begin(); Arg != F.arg_end(); ++Arg) {
        callCaptureHook(Builder, M, Arg, ShiftMap, CustTypeIdxMap, UnsignedMap);
      }
    }

    if (!FunctionIDMapper::flush(std::move(IDMap))) {
      errs() << "Failed to write function mapping!\n";
    }
    return PreservedAnalyses::none();
  }
};
} // namespace

// Register the pass for the new pass manager
llvm::PassPluginLibraryInfo getInsertFunctionCallPassPluginInfo() {
  const auto callback = [](PassBuilder &PB) {
    PB.registerPipelineStartEPCallback(
        [](ModulePassManager &MPM, OptimizationLevel) {
          MPM.addPass(InsertFunctionCallPass());
          return true;
        });
  };
  return {LLVM_PLUGIN_API_VERSION, "InsertFunctionCallPass", "0.0.1", callback};
}

// Register the plugin
extern "C" LLVM_ATTRIBUTE_WEAK PassPluginLibraryInfo llvmGetPassPluginInfo() {
  return getInsertFunctionCallPassPluginInfo();
}