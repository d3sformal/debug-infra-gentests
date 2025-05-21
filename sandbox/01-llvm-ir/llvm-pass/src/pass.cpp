#include "llvm/Pass.h"
#include "../../custom-metadata-pass/ast-meta-add/llvm-metadata.h"
#include "argMapping.hpp"
#include "constants.hpp"
#include "mapping.hpp"
#include "typeAlias.hpp"
#include "typeids.h"
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
#include <type_traits>
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

void insertArgCapturePreambleHook(IRBuilder<> &Builder, Module &M,
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
      "hook_arg_preabmle",
      FunctionType::get(Type::getVoidTy(M.getContext()),
                        {ModIdConstant->getType(), FnIdConstant->getType()},
                        false));
  Builder.CreateCall(Callee, {ModIdConstant, FnIdConstant});
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
                          const ClangMetadataToLLVMArgumentMapping &Mapping,
                          const Vec<Pair<size_t, LlcapSizeType>> &Sizes) {
  auto &Ctx = M.getContext();
  auto GetOrInsertHookFn = [&](const char *HookName, auto *TypePtr) {
    return M.getOrInsertFunction(
        HookName, FunctionType::get(Type::getVoidTy(Ctx), {TypePtr}, false));
  };

  auto ArgNum = Arg->getArgNo();
  auto *ArgT = Arg->getType();

  if (ArgT->isFloatTy()) {
    IF_VERBOSE errs() << "Inserting call f32\n";
    auto CallFloat = GetOrInsertHookFn("hook_float", Type::getFloatTy(Ctx));
    Builder.CreateCall(CallFloat, {Arg});
    return;
  }

  if (ArgT->isDoubleTy()) {
    IF_VERBOSE errs() << "Inserting call f64\n";
    auto CallDouble = GetOrInsertHookFn("hook_double", Type::getDoubleTy(Ctx));
    Builder.CreateCall(CallDouble, {Arg});
    return;
  }

  auto CallByte = GetOrInsertHookFn("hook_char", Type::getInt8Ty(Ctx));
  auto CallUnsByte = GetOrInsertHookFn("hook_uchar", Type::getInt8Ty(Ctx));
  auto CallShort = GetOrInsertHookFn("hook_short", Type::getInt8Ty(Ctx));
  auto CallUnsShort = GetOrInsertHookFn("hook_ushort", Type::getInt8Ty(Ctx));
  auto CallInt32 = GetOrInsertHookFn("hook_int32", Type::getInt32Ty(Ctx));
  auto CallUnsInt32 = GetOrInsertHookFn("hook_uint32", Type::getInt32Ty(Ctx));
  auto CallInt64 = GetOrInsertHookFn("hook_int64", Type::getInt64Ty(Ctx));
  auto CallUnsInt64 = GetOrInsertHookFn("hook_uint64", Type::getInt64Ty(Ctx));

  const Map<LlcapSizeType, Pair<FunctionCallee, FunctionCallee>>
      IntTypeSizeMap = {
          {LlcapSizeType::LLSZ_8, Pair{CallUnsByte, CallByte}},
          {LlcapSizeType::LLSZ_16, Pair{CallUnsShort, CallShort}},
          {LlcapSizeType::LLSZ_32, Pair{CallUnsInt32, CallInt32}},
          {LlcapSizeType::LLSZ_64, Pair{CallUnsInt64, CallInt64}}};
  static_assert(
      std::underlying_type_t<LlcapSizeType>(LlcapSizeType::LLSZ_8) * 8 == 8,
      "Failed basic check");

  auto IsAttrUnsgined =
      Mapping.llvmArgNoMatches(ArgNum, VSTR_LLVM_UNSIGNED_IDCS);
  auto ThisArgSize = Sizes[ArgNum].second;
  if (!isValid(ThisArgSize)) {
    errs()
        << "Encountered an invalid argument size specifier, cannot instrument";
    IF_VERBOSE {
      errs() << " arg:\n";
      Arg->dump();
    }
    errs() << "\n";
    return;
  }

  if (IntTypeSizeMap.contains(ThisArgSize)) {
    const unsigned int Bits =
        std::underlying_type_t<LlcapSizeType>(ThisArgSize) * 8;
    if (ArgT->isIntegerTy(Bits)) {
      auto &&[UnsFn, SignFn] = IntTypeSizeMap.at(ThisArgSize);
      IF_VERBOSE errs() << "Inserting call " << std::to_string(Bits)
                        << (IsAttrUnsgined ? "U\n" : "S\n");
      Builder.CreateCall(IsAttrUnsgined ? UnsFn : SignFn, {Arg});
      return;
    }
  }

  if (Mapping.llvmArgNoMatches(ArgNum, VSTR_LLVM_CXX_DUMP_STDSTRING)) {
    if (!ArgT->isPointerTy()) {
      errs() << "std::string hooks cannot handle non-pointer argument of this "
                "type yet\n";
      return;
    }

    IF_VERBOSE errs() << "Inserting call std::string\n";
    auto CallCxxString = GetOrInsertHookFn("vstr_extra_cxx__string", ArgT);
    Builder.CreateCall(CallCxxString, {Arg});
    return;
  }

  errs() << "Encountered an unknown argument size specifier "
         << std::underlying_type_t<LlcapSizeType>(ThisArgSize) << '\n';
  IF_VERBOSE Arg->dump();
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

ClangMetadataToLLVMArgumentMapping
getFullyRegisteredArgMapping(Function &Fn, IdxMappingInfo &IdxInfo) {
  ClangMetadataToLLVMArgumentMapping Mapping(Fn, IdxInfo);
  Mapping.registerCustomTypeIndicies(VSTR_LLVM_CXX_DUMP_STDSTRING,
                                     LlcapSizeType::LLSZ_CUSTOM);
  Mapping.registerCustomTypeIndicies(
      VSTR_LLVM_UNSIGNED_IDCS,
      LlcapSizeType::LLSZ_INVALID); // invalid size -> indeterminate size means
                                    // that this type index is just a "flag" and
                                    // has no effect on the final size read
  return Mapping;
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
  IdxMappingInfo m_seps;

public:
  FunctionEntryInsertion(Module &M, IdxMappingInfo Seps)
      : m_fnIdMap(M.getModuleIdentifier()), m_module(M), m_seps(Seps) {}

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
      ClangMetadataToLLVMArgumentMapping Mapping =
          getFullyRegisteredArgMapping(Fn, m_seps);
      const auto FunId = m_fnIdMap.addFunction(DemangledName, Mapping);

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
  llcap::ModuleId m_moduleId;
  Map<Str, llcap::FunctionId> &m_traced_functions;
  IdxMappingInfo m_idxInfo;

public:
  ArgumentInstrumentation(
      Module &M, Pair<llcap::ModuleId, Map<Str, uint32_t>> &TracedFunctions,
      IdxMappingInfo Info)
      : m_module(M), m_moduleId(TracedFunctions.first),
        m_traced_functions(TracedFunctions.second), m_idxInfo(Info) {}

  void run() override {
    for (Function &Fn : m_module) {
      StringRef MangledName = Fn.getFunction().getName();
      Str DemangledName = llvm::demangle(MangledName);

      auto FnId = m_traced_functions.find(DemangledName);
      if (FnId == m_traced_functions.end()) {
        IF_DEBUG errs() << "Skipping fn " << DemangledName << "\n";
        continue;
      }
      IF_VERBOSE errs() << "Instrumenting fn " << DemangledName << "\n";

      BasicBlock &EntryBB = Fn.getEntryBlock();
      IRBuilder<> Builder(&EntryBB.front());

      ClangMetadataToLLVMArgumentMapping Mapping =
          getFullyRegisteredArgMapping(Fn, m_idxInfo);
      insertArgCapturePreambleHook(Builder, m_module, m_moduleId, FnId->second);

      for (auto *Arg = Fn.arg_begin(); Arg != Fn.arg_end(); ++Arg) {
        insertArgCaptureHook(Builder, m_module, Arg, Mapping,
                             Mapping.getArgumentSizeTypes());
      }
    }
  }

  bool finish() override { return true; }
};

Maybe<Pair<llcap::ModuleId, Map<Str, uint32_t>>>
collectTracedFunctionsForModule(Module &M) {
  Map<Str, uint32_t> Map;

  Maybe<llcap::ModuleId> NumericModId = NONE;

  const Str &Path = args::TargetsFilePath.getValue();
  if (Path.empty()) {
    return NONE;
  }

  std::ifstream Targets(Path, std::ios::binary);
  if (!Targets) {
    errs() << "Could not open targets file @ " << Path << "\n";
    return NONE;
  }

  auto NextPosMove = [](Str &Data, u64 &Pos, auto &NextPos, const char *Msg) {
    constexpr char SEP = '\x00';
    NextPos = Data.find(SEP, Pos);
    if (NextPos == Str::npos) {
      errs() << "functions-to-trace mapping: format invalid (" << Msg << ")!\n";
      return false;
    }
    return true;
  };

  Str Data;
  u64 Pos = 0;
  while (std::getline(Targets, Data, '\n')) {
    if (Data.empty()) {
      IF_DEBUG errs() << "Skip empty\n";
      Pos = 0;
      continue;
    }

    u64 NextPos = Pos;
    if (!NextPosMove(Data, Pos, NextPos, "mod id")) {
      return NONE;
    }

    Str ModId = Data.substr(Pos, NextPos - Pos);
    if (ModId != M.getModuleIdentifier()) {
      IF_DEBUG errs() << "Skip on module mismatch " << ModId << "\n";
      Pos = 0;
      continue;
    }

    Pos = NextPos + 1;

    if (!NextPosMove(Data, Pos, NextPos, "mod id n")) {
      return NONE;
    }

    Maybe<u64> ModIdRes =
        tryParse<llcap::ModuleId>(Data.substr(Pos, NextPos - Pos));
    if (!ModIdRes) {
      IF_DEBUG errs()
          << "functions-to-trace mapping: format invalid (mod id n)\n";
      return NONE;
    }
    NumericModId = ModIdRes;

    Pos = NextPos + 1;
    if (!NextPosMove(Data, Pos, NextPos, "fn id")) {
      return NONE;
    }

    auto FnId = Data.substr(Pos, NextPos - Pos);

    Pos = NextPos + 1;
    auto FnIdNumeric = tryParse<llcap::ModuleId>(Data.substr(Pos));
    if (!FnIdNumeric) {
      IF_DEBUG errs()
          << "functions-to-trace mapping: format invalid (fn id n)\n";
      return NONE;
    }

    IF_VERBOSE errs() << "Add \"to trace\" " << FnId << ", ID: " << *FnIdNumeric
                      << "\n";
    Map[FnId] = *FnIdNumeric;
    Pos = 0;
  }

  if (!NumericModId) {
    return NONE;
  }

  return std::make_pair(*NumericModId, Map);
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
      auto TracedFns = collectTracedFunctionsForModule(M);
      if (!TracedFns) {
        errs() << "Failed to parse instrumentation targets\n";
        return PreservedAnalyses::none();
      }
      ArgumentInstrumentation Work(M, *TracedFns, MappingInfo);
      Work.run();

      if (!Work.finish()) {
        errs() << "Instrumentation failed - ArgumentInstrumentation!\n";
        exit(1);
      }
    } else {
      IF_VERBOSE errs() << "Instrumenting fn entry...\n";
      FunctionEntryInsertion Work(M, MappingInfo);
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