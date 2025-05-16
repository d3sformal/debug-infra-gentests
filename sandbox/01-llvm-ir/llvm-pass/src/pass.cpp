#include "llvm/Pass.h"
#include "../../custom-metadata-pass/ast-meta-add/llvm-metadata.h"
#include "argMapping.hpp"
#include "constants.hpp"
#include "mapping.hpp"
#include "typeAlias.hpp"
#include "utility.hpp"
#include "verbosity.hpp"
#include "llvm/ADT/APInt.h"
#include "llvm/ADT/SmallVector.h"
#include <llvm/ADT/StringRef.h>
#include <llvm/Analysis/TargetLibraryInfo.h>
#include <llvm/Demangle/Demangle.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/DerivedTypes.h>
#include "llvm/IR/Function.h"
#include <llvm/IR/GlobalVariable.h>
#include "llvm/IR/IRBuilder.h"
#include <llvm/IR/LLVMContext.h>
#include "llvm/IR/Metadata.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/PassManager.h"
#include <llvm/IR/Type.h>
#include <llvm/Passes/OptimizationLevel.h>
#include "llvm/Passes/PassBuilder.h"
#include "llvm/Passes/PassPlugin.h"
#include "llvm/Support/Casting.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/raw_ostream.h"
#include <fstream>
#include <ios>
#include <string>
#include <utility>

using namespace llvm;

namespace {
namespace args {
enum class InstrumentationType : u8 { Call, Arg };

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
cl::opt<InstrumentationType> InstrumentationType(
    cl::desc("Choose instrumentation type:"),
    cl::values(
        llvm ::cl ::OptionEnumValue{.Name = "Call",
                                    .Value = int(InstrumentationType::Call),
                                    .Description = "Call tracing"},
        llvm ::cl ::OptionEnumValue{.Name = "Arg",
                                    .Value = int(InstrumentationType::Arg),
                                    .Description = "Argument tracing"}));

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
bool isStdFnDanger(const StringRef Mangled) {
  return Mangled.starts_with("_ZNSt") || Mangled.starts_with("_ZZNSt") ||
         Mangled.starts_with("_ZSt") || Mangled.starts_with("_ZNSo") ||
         Mangled.starts_with("_ZNSi") || Mangled.starts_with("_ZNSe") ||
         Mangled.starts_with("_ZNSc") || Mangled.starts_with("_ZNSs") ||
         Mangled.starts_with("_ZNSa") || Mangled.starts_with("__");
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

    if (auto *Op = dyn_cast_if_present<MDString>(N->getOperand(0));
        Op == nullptr) {
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

Maybe<Str> getMetadataStrVal(NamedMDNode *Node) {
  if (Node == nullptr || Node->getNumOperands() == 0) {
    return NONE;
  }
  MDNode *Inner = Node->getOperand(0);

  if (Inner->getNumOperands() == 0) {
    return NONE;
  }

  if (auto *Op = dyn_cast_if_present<MDString>(Inner->getOperand(0));
      Op != nullptr) {
    return Op->getString().str();
  }
  return NONE;
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
    IF_VERBOSE errs() << "Inserting call 8" << (IsAttrUnsgined ? "U" : "S")
                      << '\n';
    Builder.CreateCall(IsAttrUnsgined ? CallUnsByte : CallByte, {Arg});
  } else if (ArgT->isIntegerTy(16)) {
    IF_VERBOSE errs() << "Inserting call 16" << (IsAttrUnsgined ? "U" : "S")
                      << '\n';
    Builder.CreateCall(IsAttrUnsgined ? CallUnsShort : CallShort, {Arg});
  } else if (ArgT->isIntegerTy(32)) {
    IF_VERBOSE errs() << "Inserting call 32" << (IsAttrUnsgined ? "U" : "S")
                      << '\n';
    Builder.CreateCall(IsAttrUnsgined ? CallUnsInt32 : CallInt32, {Arg});
  } else if (ArgT->isIntegerTy(64)) {
    IF_VERBOSE errs() << "Inserting call 64" << (IsAttrUnsgined ? "U" : "S")
                      << '\n';
    Builder.CreateCall(IsAttrUnsgined ? CallUnsInt64 : CallInt64, {Arg});
  } else if (ArgT->isFloatTy()) {
    IF_VERBOSE errs() << "Inserting call f32\n";
    Builder.CreateCall(CallFloat, {Arg});
  } else if (ArgT->isDoubleTy()) {
    IF_VERBOSE errs() << "Inserting call f64\n";
    Builder.CreateCall(CallDouble, {Arg});
  } else if (Mapping.llvmArgNoMatches(ArgNum, VSTR_LLVM_CXX_DUMP_STDSTRING)) {
    IF_VERBOSE errs() << "Inserting call std::string\n";
    auto CallCxxString = M.getOrInsertFunction(
        "vstr_extra_cxx__string",
        FunctionType::get(Type::getVoidTy(Ctx), {Arg->getType()}, false));
    Builder.CreateCall(CallCxxString, {Arg});
  } else {
    IF_VERBOSE errs() << "Skipping Argument\n";
    IF_DEBUG Arg->dump();
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

void insertArgTracingHooks(Module &M, Function &Fn, IdxMappingInfo &IdxInfo) {
  BasicBlock &EntryBB = Fn.getEntryBlock();
  IRBuilder<> Builder(&EntryBB.front());
  ClangMetadataToLLVMArgumentMapping Mapping(Fn, IdxInfo);
  Mapping.registerCustomTypeIndicies(VSTR_LLVM_CXX_DUMP_STDSTRING);
  Mapping.registerCustomTypeIndicies(VSTR_LLVM_UNSIGNED_IDCS);

  for (auto *Arg = Fn.arg_begin(); Arg != Fn.arg_end(); ++Arg) {
    insertArgCaptureHook(Builder, M, Arg, Mapping);
  }
}

bool instrumentArgs() {
  return args::InstrumentationType.getValue() == args::InstrumentationType::Arg;
}

const char *MappingParseGuideMetaKey = "LLCAP-CLANG-LLVM-MAP-PRSGD";
const char *MappingInvlIdxMetaKey = "LLCAP-CLANG-LLVM-MAP-INVLD-IDX";
const char *MappingMetaKey = "LLCAP-CLANG-LLVM-MAP-DATA";

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
        if (MDNode *N = Fn.getMetadata(MappingMetaKey)) {
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
        for (auto &&T : {Type::FloatTyID, Type::IntegerTyID, Type::DoubleTyID,
                         Type::PointerTyID}) {
          AllowedTypes.insert(T);
        }
      }

      bool Viable = !Fn.arg_empty();
      if (Viable) {
        for (auto *Arg = Fn.arg_begin(); Arg != Fn.arg_end(); ++Arg) {
          if (!AllowedTypes.contains(Arg->getType()->getTypeID())) {
            Viable = false;
            break;
          }
        }
      }
      // TODO handle viability
      const auto FunId = m_fnIdMap.addFunction(DemangledName);

      insertFnEntryHook(Builder, m_module, m_fnIdMap.getModuleMapIntId(),
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
  IdxMappingInfo m_idxInfo;

public:
  ArgumentInstrumentation(Module &M, Set<Str> &TracedFunctions,
                          IdxMappingInfo Info)
      : m_module(M), m_traced_functions(TracedFunctions), m_idxInfo(Info) {}

  void run() override {
    for (Function &Fn : m_module) {
      StringRef MangledName = Fn.getFunction().getName();
      Str DemangledName = llvm::demangle(MangledName);

      if (!m_traced_functions.contains(DemangledName)) {
        IF_DEBUG errs() << "Skipping fn " << DemangledName << "\n";
        continue;
      }
      IF_VERBOSE errs() << "Instrumenting fn " << DemangledName << "\n";
      insertArgTracingHooks(m_module, Fn, m_idxInfo);
    }
  }

  bool finish() override { return true; }
};

auto collectTracedFunctionsForModule(Module &M) {
  using RetT = Set<Str>;
  RetT Result;

  const Str &Path = args::TargetsFilePath.getValue();
  if (Path.empty()) {
    return Result;
  }

  std::ifstream Targets(Path, std::ios::binary);
  if (!Targets) {
    errs() << "Could not open targets file @ " << Path << "\n";
    return Result;
  }

  Str Data;
  while (std::getline(Targets, Data, '\n')) {
    if (Data.empty()) {
      IF_DEBUG errs() << "Skip empty\n";
      continue;
    }

    auto Pos = Data.find('\x00');
    if (Pos == Str::npos) {
      errs() << "functions-to-trace mapping: format invalid (fn id)!\n";
      return RetT();
    }

    auto ModId = Data.substr(0, Pos);
    if (ModId != M.getModuleIdentifier()) {
      IF_DEBUG errs() << "Skip on module mismatch " << ModId << "\n";
      continue;
    }

    auto FnId = Data.substr(Pos + 1);
    IF_VERBOSE errs() << "Add \"to trace\" " << FnId << "\n";
    Result.insert(FnId);
  }

  return Result;
}

Maybe<IdxMappingInfo> getIdxMappingInfo(Module &M) {
  IdxMappingInfo Result;

  if (auto MbStr =
          getMetadataStrVal(M.getNamedMetadata(MappingParseGuideMetaKey));
      MbStr && MbStr->size() == 3) {
    Result.primary = MbStr->at(0);
    Result.group = MbStr->at(1);
    Result.argParamPair = MbStr->at(2);
    Result.custom = VSTR_LLVM_CXX_SINGLECHAR_SEP;
  } else {
    llvm::errs() << "Module missing parse guide\n";
    return NONE;
  }

  // TODO: make mandatory
  Result.invalidIndexValue = 0xFFFFFFFFFFFFFFFF;
  // try get string from metadata
  if (auto MbStr = getMetadataStrVal(M.getNamedMetadata(MappingInvlIdxMetaKey));
      MbStr) {
    // try parse u64 from the string
    if (auto Parsed = tryParse<u64>(*MbStr); Parsed) {
      Result.invalidIndexValue = *Parsed;
    } else {
      llvm::errs() << "Module invalid index hint could not be parsed\n";
    }
  } else {
    llvm::errs() << "Module missing invalid index hint\n";
  }

  IF_DEBUG llvm::errs() << "Module Index Map parsing OK\n";
  return Result;
}

struct InsertFunctionCallPass : public PassInfoMixin<InsertFunctionCallPass> {

  PreservedAnalyses run(Module &M, ModuleAnalysisManager &AM) {
    verbose(args::Verbose.getValue(), args::Verbose.getValue());
    debug(args::Debug.getValue(), args::Debug.getValue());

    IF_VERBOSE errs() << "Running pass on module " << M.getModuleIdentifier()
                      << "\n";
    IdxMappingInfo MappingInfo;
    if (auto MbInfo = getIdxMappingInfo(M); MbInfo) {
      MappingInfo = *MbInfo;
    } else {
      IF_VERBOSE errs() << "Skipping entire module " + M.getModuleIdentifier()
                        << '\n';
      // not really sure if "all" collides with other modules or not? => remain
      // pessimistic
      return PreservedAnalyses::none();
    }

    if (instrumentArgs()) {
      IF_VERBOSE errs() << "Instrumenting args...\n";
      Set<Str> TracedFns = collectTracedFunctionsForModule(M);
      ArgumentInstrumentation Work(M, TracedFns, MappingInfo);
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

namespace {
// Register the pass for the new pass manager
llvm::PassPluginLibraryInfo getInsertFunctionCallPassPluginInfo() {
  const auto Callback = [](PassBuilder &PB) {
    PB.registerPipelineStartEPCallback(
        [](ModulePassManager &MPM, OptimizationLevel) {
          MPM.addPass(InsertFunctionCallPass());
          return true;
        });
  };
  return {.APIVersion = LLVM_PLUGIN_API_VERSION,
          .PluginName = "InsertFunctionCallPass",
          .PluginVersion = "0.0.1",
          .RegisterPassBuilderCallbacks = Callback};
}
} // namespace

// Register the plugin
extern "C" LLVM_ATTRIBUTE_WEAK PassPluginLibraryInfo llvmGetPassPluginInfo() {
  return getInsertFunctionCallPassPluginInfo();
}