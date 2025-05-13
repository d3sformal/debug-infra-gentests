#include "llvm/Pass.h"
#include "../../custom-metadata-pass/ast-meta-add/llvm-metadata.h"
#include "argMapping.hpp"
#include "constants.hpp"
#include "mapping.hpp"
#include "typeAlias.hpp"
#include "verbosity.hpp"
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
#include "llvm/Support/raw_ostream.h"
#include <fstream>
#include <ios>
#include <llvm/ADT/StringRef.h>
#include <llvm/Analysis/TargetLibraryInfo.h>
#include <llvm/Demangle/Demangle.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/GlobalVariable.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Type.h>
#include <llvm/Passes/OptimizationLevel.h>
#include <string>
#include <utility>

using namespace llvm;

static const char *UnsignedAttrKind = "VSTR-param-attr-unsigned";

namespace {
namespace args {
enum InstrumentationType { Call, Arg };

// -mllvm -llcap-filter-by-mangled
cl::opt<bool>
    MangleFilter("llcap-filter-by-mangled",
                 cl::desc("Filter functions by their mangled names instead of "
                          "their position within the AST"));
// -mllvm -llcap-verbose
cl::opt<bool> Verbose("llcap-verbose", cl::desc("Verbose output"));
// -mllvm -llcap-debug
cl::opt<bool> Debug("llcap-debug", cl::desc("Debugging output"));
// -mllvm -llcap-mapdir
cl::opt<std::string>
    MapFilesDirectory("llcap-mapdir",
                      cl::desc("Output directory for function ID maps"));
// -mllvm -Call
// -mllvm -Arg
cl::opt<InstrumentationType>
    InstrumentationType(cl::desc("Choose instrumentation type:"),
                        cl::values(clEnumVal(Call, "Call tracing"),
                                   clEnumVal(Arg, "Argument tracing")));

// -mllvm -llcap-fn-targets-file
cl::opt<std::string>
    TargetsFilePath("llcap-fn-targets-file",
                    cl::desc("Path to a file containing the module IDs and "
                             "function IDs of functions to be instrumented"));

// -mllvm -llcap-argcapture-file
cl::opt<std::string> ArgCaptureIdMapPath(
    "llcap-argcapture-file",
    cl::desc("Output file where argument tracing IDs are written"));
} // namespace args

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

bool isStdFnBasedOnMetadata(const Function &Fn,
                            const std::string &DemangledName,
                            const StringRef MangledName) {
  IF_VERBOSE errs() << "Metadata of function " << DemangledName << '\n';
  if (MDNode *N = Fn.getMetadata(VSTR_LLVM_NON_SYSTEMHEADER_FN_KEY)) {
    if (N->getNumOperands() == 0) {
      IF_VERBOSE errs() << "Warning! Unexpected metadata node with no "
                           "operands! Function: "
                        << MangledName << ' ' << DemangledName << '\n';
    }

    if (MDString *op = dyn_cast_if_present<MDString>(N->getOperand(0));
        op == nullptr) {
      IF_VERBOSE {
        errs() << "Invalid metadata for node in function: " << MangledName
               << ' ' << DemangledName << "\nNode:\n";
        N->dumpTree();
      }
    }
    return false;
  }
  return true;
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

void insertFnEntryHook(IRBuilder<> &Builder, Module &M,
                       llcap::ModuleId ModuleIntId,
                       llcap::FunctionId FunctionIntId) {
  static_assert(sizeof(llcap::FunctionId) ==
                4); // this does not imply incorrectness, just that everything
                    // must be checked
  auto *FnIdConstant = ConstantInt::get(
      M.getContext(), APInt(sizeof(llcap::FunctionId) * 8, FunctionIntId));
  static_assert(sizeof(llcap::ModuleId) == 4);
  auto *ModIdConstant = ConstantInt::get(
      M.getContext(), APInt(sizeof(llcap::ModuleId) * 8, ModuleIntId));
  auto Callee = M.getOrInsertFunction(
      "hook_start",
      FunctionType::get(Type::getVoidTy(M.getContext()),
                        {ModIdConstant->getType(), FnIdConstant->getType()},
                        false));
  Builder.CreateCall(Callee, {ModIdConstant, FnIdConstant});
}
// terminology:
// LLVM Argument Index = 0-based index of an argument as seen directly in the
// LLVM IR

//  AST Argument Index  = 0-based idx as seen in the Clang AST

// key differences accounted for: this pointer & sret arguments (returning a
// struct in a register)

void insertArgCaptureHook(IRBuilder<> &Builder, Module &M, Argument *Arg,
                          const ClangMetadataToLLVMArgumentMapping &Mapping) {
  auto ArgNum = Arg->getArgNo();
  auto &Ctx = M.getContext();
  auto IsAttrUnsgined =
      Mapping.llvmArgNoMatches(ArgNum, VSTR_LLVM_UNSIGNED_IDCS);
  auto *ArgT = Arg->getType();
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
  } else if (Mapping.llvmArgNoMatches(ArgNum, VSTR_LLVM_CXX_DUMP_STDSTRING)) {
    auto CallCxxString = M.getOrInsertFunction(
        "vstr_extra_cxx__string",
        FunctionType::get(Type::getVoidTy(Ctx), {Arg->getType()}, false));
    Builder.CreateCall(CallCxxString, {Arg});
  } else {
    IF_VERBOSE errs() << "Skipping Argument\n";
    IF_VERBOSE Arg->dump();
  }
}

bool isStdFn(const Function &Fn, const Str &DemangledName,
             const StringRef Name) {
  if (args::MangleFilter.getValue() && isStdFnDanger(Name)) {
    return true;
  }
  if (!args::MangleFilter.getValue() &&
      isStdFnBasedOnMetadata(Fn, DemangledName, Name)) {
    return true;
  }
  return false;
}

void insertArgTracingHooks(Module &M, Function &Fn) {
  BasicBlock &EntryBB = Fn.getEntryBlock();
  IRBuilder<> Builder(&EntryBB.front());
  ClangMetadataToLLVMArgumentMapping Mapping(Fn);
  Mapping.registerCustomTypeIndicies(VSTR_LLVM_CXX_DUMP_STDSTRING);
  Mapping.registerCustomTypeIndicies(VSTR_LLVM_UNSIGNED_IDCS);

  for (auto *Arg = Fn.arg_begin(); Arg != Fn.arg_end(); ++Arg) {
    insertArgCaptureHook(Builder, M, Arg, Mapping);
  }
}

bool instrumentArgs() {
  return args::InstrumentationType.getValue() == args::InstrumentationType::Arg;
}

class Instrumentation {
public:
  virtual ~Instrumentation() = default;
  virtual bool prepare() { return true; };
  virtual void run() = 0;
  virtual bool finish() = 0;
};

class FunctionEntryInsertion : public Instrumentation {
private:
  FunctionIDMapper m_fnIdMap;
  Module &m_module;

public:
  FunctionEntryInsertion(Module &M)
      : m_fnIdMap(M.getModuleIdentifier()), m_module(M) {}

  bool prepare() override {
    return args::MapFilesDirectory.getValue().length() != 0;
  }

  void run() override {
    for (Function &Fn : m_module) {

      // Skip library functions
      StringRef MangledName = Fn.getFunction().getName();
      Str DemangledName = llvm::demangle(MangledName);

      IF_DEBUG {
        if (MDNode *N = Fn.getMetadata("LLCAP-CLANG-LLVM-MAP-DATA")) {
          errs() << DemangledName << ": \n";
          N->dumpTree();
        }
      }

      if (isStdFn(Fn, DemangledName, MangledName)) {
        continue;
      }

      BasicBlock &EntryBB = Fn.getEntryBlock();
      IRBuilder<> Builder(&EntryBB.front());

      // TODO: this needs improvement (Vector types, ...)
      Set<llvm::Type::TypeID> AllowedTypes;
      {
        using llvm::Type;
        for (auto &&t : {Type::FloatTyID, Type::IntegerTyID, Type::DoubleTyID,
                         Type::PointerTyID}) {
          AllowedTypes.insert(t);
        }
      }

      bool viable = !Fn.arg_empty();
      if (viable) {
        for (auto *Arg = Fn.arg_begin(); Arg != Fn.arg_end(); ++Arg) {
          if (AllowedTypes.find(Arg->getType()->getTypeID()) ==
              AllowedTypes.end()) {
            viable = false;
            break;
          }
        }
      }
      // TODO handle viability
      const auto FunId = m_fnIdMap.addFunction(DemangledName);

      insertFnEntryHook(Builder, m_module, m_fnIdMap.GetModuleMapIntId(),
                        FunId);
    }
  }

  bool finish() override {
    return FunctionIDMapper::flush(std::move(m_fnIdMap),
                                   args::MapFilesDirectory.getValue());
  }
};

class ArgumentInstrumentation : public Instrumentation {
  Module &m_module;
  Set<Str> &m_traced_functions;

public:
  ArgumentInstrumentation(Module &M, Set<Str> &traced_functions)
      : m_module(M), m_traced_functions(traced_functions) {}

  void run() override {
    for (Function &Fn : m_module) {
      StringRef MangledName = Fn.getFunction().getName();
      Str DemangledName = llvm::demangle(MangledName);

      if (m_traced_functions.find(DemangledName) == m_traced_functions.end()) {
        IF_VERBOSE errs() << "Skipping fn " << DemangledName << "\n";
        continue;
      }
      IF_VERBOSE errs() << "Instrumenting fn " << DemangledName << "\n";
      insertArgTracingHooks(m_module, Fn);
    }
  }

  bool finish() override { return true; }
};

auto collectTracedFunctionsForModule(Module &M) {
  using RetT = Set<Str>;
  RetT result;

  Str Path = args::TargetsFilePath.getValue();
  if (Path.empty()) {
    return result;
  }

  std::ifstream Targets(Path, std::ios::binary);
  if (!Targets) {
    errs() << "Could not open targets file @ " << Path << "\n";
    return result;
  }

  Str Data;
  while (std::getline(Targets, Data, '\n')) {
    if (Data.empty()) {
      IF_VERBOSE errs() << "Skip empty\n";
      continue;
    }

    auto Pos = Data.find('\x00');
    if (Pos == Str::npos) {
      errs() << "functions-to-trace mapping: format invalid (fn id)!\n";
      return RetT();
    }

    auto ModId = Data.substr(0, Pos);
    if (ModId != M.getModuleIdentifier()) {
      IF_VERBOSE errs() << "skip on module mismatch " << ModId << "\n";
      continue;
    }

    auto FnId = Data.substr(Pos + 1);
    IF_VERBOSE errs() << "Add " << FnId << "\n";
    result.insert(FnId);
  }

  return result;
}

struct InsertFunctionCallPass : public PassInfoMixin<InsertFunctionCallPass> {

  PreservedAnalyses run(Module &M, ModuleAnalysisManager &AM) {
    verbose(args::Verbose.getValue(), args::Verbose.getValue());
    debug(args::Debug.getValue(), args::Debug.getValue());

    IF_VERBOSE errs() << "Running pass on module " << M.getModuleIdentifier()
                      << "\n";

    IF_DEBUG {
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
    }

    if (instrumentArgs()) {
      IF_VERBOSE errs() << "Instrumenting args...\n";
      Set<Str> TracedFns = collectTracedFunctionsForModule(M);
      ArgumentInstrumentation Work(M, TracedFns);
      Work.run();

      if (!Work.finish()) {
        errs() << "Instrumentation failed - ArgumentInstrumentation!\n";
        exit(1);
      }
    } else {
      IF_VERBOSE errs() << "Instrumenting fn entry...\n";
      FunctionEntryInsertion Work(M);
      Work.run();

      if (!Work.finish()) {
        errs() << "Instrumentation failed - FunctionEntryInsertion!\n";
        errs() << "FunctionEntryInsertion requires -mllvm -llcap-mapdir DIR "
                  "directory!\n";
        exit(1);
      }
    }
    IF_VERBOSE errs() << "Instrumentation DONE!\n";

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