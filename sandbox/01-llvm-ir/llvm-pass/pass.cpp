#include "llvm/Pass.h"
#include "llvm/IR/Function.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/PassManager.h"
#include "llvm/Passes/PassBuilder.h"
#include "llvm/Passes/PassPlugin.h"
#include <llvm/ADT/StringRef.h>
#include <llvm/Demangle/Demangle.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/GlobalVariable.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Type.h>
#include <llvm/Passes/OptimizationLevel.h>

#include <bits/stdc++.h>
#include <llvm/Analysis/TargetLibraryInfo.h>

using namespace llvm;

namespace {

// there is no way to tell built-ins from user functions in the IR
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

void insertFnEntryLog(IRBuilder<> &Builder, Function &F, Module &M,
                      GlobalVariable *Message) {
  auto *IRStrPtr = Builder.CreateBitCast(Message, Builder.getPtrTy());
  auto Callee = M.getOrInsertFunction(
      "hook_start", FunctionType::get(Type::getVoidTy(M.getContext()),
                                      {Builder.getPtrTy()}, false));
  Builder.CreateCall(Callee, {IRStrPtr});
}
void callCaptureHook(IRBuilder<> &Builder, Module &M, Argument *Arg) {
  auto &Ctx = M.getContext();
  auto ArgT = Arg->getType();

  auto CallInt32 = M.getOrInsertFunction(
      "hook_int32",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getInt32Ty(Ctx)}, false));
  auto CallInt64 = M.getOrInsertFunction(
      "hook_int64",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getInt64Ty(Ctx)}, false));
  auto CallFloat = M.getOrInsertFunction(
      "hook_float",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getFloatTy(Ctx)}, false));
  auto CallDouble = M.getOrInsertFunction(
      "hook_double",
      FunctionType::get(Type::getVoidTy(Ctx), {Type::getDoubleTy(Ctx)}, false));

  if (ArgT->isIntegerTy(32)) {
    Builder.CreateCall(CallInt32, {Arg});
  } else if (ArgT->isIntegerTy(64)) {
    Builder.CreateCall(CallInt64, {Arg});
  } else if (ArgT->isFloatTy()) {
    Builder.CreateCall(CallFloat, {Arg});
  } else if (ArgT->isDoubleTy()) {
    Builder.CreateCall(CallDouble, {Arg});
  } else {
    outs() << "Skipping\n";
    Arg->dump();
  }
}

struct InsertFunctionCallPass : public PassInfoMixin<InsertFunctionCallPass> {

  PreservedAnalyses run(Module &M, ModuleAnalysisManager &AM) {
    LLVMContext &Context = M.getContext();

    outs() << "Running pass!\n";

    auto &FAM =
        AM.getResult<llvm::FunctionAnalysisManagerModuleProxy>(M).getManager();

    for (auto &F : M) {
      LibFunc OutFunc;
      auto &TLI = FAM.getResult<TargetLibraryAnalysis>(F);
      // for skipping builtin functions
      std::set<StringRef> Builtins;
      for (auto &F2 : M) {
        if (TLI.getLibFunc(F2, OutFunc)) {
          Builtins.insert(F2.getFunction().getName());
        }
      }

      // Skip builtins
      if (F.isDeclaration()) {
        continue;
      }

      // Skip library functions
      auto Name = F.getFunction().getName();
      if (isStdFnDanger(Name)) {
        continue;
      }

      auto DemangledName = llvm::demangle(Name);
      auto *IRGlobalDemangledName =
          createGlobalStr(M, DemangledName, F.getName().str() + "string");

      BasicBlock &EntryBB = F.getEntryBlock();
      IRBuilder<> Builder(&EntryBB.front());
      insertFnEntryLog(Builder, F, M, IRGlobalDemangledName);

      // TODO: record parameters
      std::set<llvm::Type::TypeID> AllowedTypes;
      {
        using llvm::Type;
        for (auto &&t :
             {Type::FloatTyID, Type::IntegerTyID, Type::DoubleTyID}) {
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

      outs() << "argument dump for " << DemangledName << '\n';
      for (auto Arg = F.arg_begin(); Arg != F.arg_end(); ++Arg) {
        callCaptureHook(Builder, M, Arg);
      }
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