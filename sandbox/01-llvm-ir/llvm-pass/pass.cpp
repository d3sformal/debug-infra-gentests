#include "llvm/Pass.h"
#include "llvm/IR/Function.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/PassManager.h"
#include "llvm/Passes/PassBuilder.h"
#include "llvm/Passes/PassPlugin.h"
#include <llvm/ADT/StringRef.h>
#include <llvm/Demangle/Demangle.h>
#include <llvm/Passes/OptimizationLevel.h>

#include <bits/stdc++.h>
#include <llvm/Analysis/TargetLibraryInfo.h>

using namespace llvm;

namespace {

bool isStdFnDanger(const StringRef mangled) {
  return mangled.starts_with("_ZNSt") || mangled.starts_with("_ZZNSt") ||
         mangled.starts_with("_ZSt") || mangled.starts_with("_ZNSo") ||
         mangled.starts_with("_ZNSi") || mangled.starts_with("_ZNSe") ||
         mangled.starts_with("_ZNSc") || mangled.starts_with("_ZNSs") ||
         mangled.starts_with("_ZNSa") || mangled.starts_with("__");
}

struct InsertFunctionCallPass : public PassInfoMixin<InsertFunctionCallPass> {

  PreservedAnalyses run(Module &M, ModuleAnalysisManager &AM) {
    LLVMContext &Context = M.getContext();
    outs() << "Ran Pass!\n";
    // Declare or create the function to insert
    Function *HookFunc = M.getFunction("my_hook");
    if (!HookFunc) {
      FunctionType *HookType =
          FunctionType::get(Type::getVoidTy(Context), false);
      HookFunc =
          Function::Create(HookType, Function::ExternalLinkage, "my_hook", M);
    }

    // does not work: this pass precedes the target library analysis pass -> TLI
    // is not available

    // IDEA: modify LLVM IR to contain this information?

    auto &FAM =
        AM.getResult<llvm::FunctionAnalysisManagerModuleProxy>(M).getManager();
    // Iterate through functions
    for (Function &F : M) {
      auto &TLI = FAM.getResult<TargetLibraryAnalysis>(F);

      LibFunc func;
      // for skipping builtin functions
      std::set<StringRef> builtins;
      for (auto &F2 : M) {
        if (TLI.getLibFunc(F2, func)) {
          builtins.insert(F2.getFunction().getName());
          // outs() << '\t' << F2.getFunction().getName() << '\n';
        }
      }

      auto name = F.getFunction().getName();
      if (F.isDeclaration() || isStdFnDanger(name))
        continue; // Skip library functions (and templates that use std lirbary
                  // types...)

      auto demangled = llvm::demangle(name);
      outs() << name << '\n';
      outs() << demangled << '\n';
      BasicBlock &EntryBB = F.getEntryBlock();
      IRBuilder<> Builder(&EntryBB.front()); // Insert at the start
      Builder.CreateCall(HookFunc);
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