#include "llvm/Pass.h"
#include "../../custom-metadata-pass/ast-meta-add/llvm-metadata.h"
#include "constants.hpp"
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

#include "mapping.hpp"
#include "typeAlias.hpp"

using namespace llvm;

const char *UNSIGNED_ATTR_KIND = "VSTR-param-attr-unsigned";

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

#define IF_VERBOSE if (args::Verbose.getValue())
#define IF_DEBUG if (args::Debug.getValue())

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

void insertArgCaptureHook(IRBuilder<> &Builder, Module &M, Argument *Arg,
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

bool isStdFn(const Function &Fn, const Str &DemangledName,
             const StringRef Name) {
  if (args::MangleFilter.getValue() && isStdFnDanger(Name)) {
    return true;
    ;
  } else if (!args::MangleFilter.getValue() &&
             isStdFnBasedOnMetadata(Fn, DemangledName, Name)) {
    return true;
  }
  return false;
}

void insertArgTracingHooks(Module &M, Function &Fn) {
  BasicBlock &EntryBB = Fn.getEntryBlock();
  IRBuilder<> Builder(&EntryBB.front());

  bool InstanceMember = Fn.getMetadata(VSTR_LLVM_CXX_THISPTR) != nullptr;

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
  const auto ShiftMap = getSretArgumentShiftVec(Fn);

  auto CustTypeIdcs =
      getCustomTypeIndicies(VSTR_LLVM_CXX_DUMP_STDSTRING, Fn, InstanceMember);
  IF_VERBOSE {
    llvm::errs() << "CustType indicies: ";
    if (CustTypeIdcs) {
      for (auto &&i : *CustTypeIdcs) {
        llvm::errs() << i << " ";
      }
    }
    llvm::errs() << '\n';
  }
  auto CustTypeIdxMap =
      CustTypeIdcs
          ? std::optional(std::set(CustTypeIdcs->begin(), CustTypeIdcs->end()))
          : std::nullopt;

  auto CustUnsignedIdcs =
      getCustomTypeIndicies(VSTR_LLVM_UNSIGNED_IDCS, Fn, InstanceMember);
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

  for (auto Arg = Fn.arg_begin(); Arg != Fn.arg_end(); ++Arg) {
    insertArgCaptureHook(Builder, M, Arg, ShiftMap, CustTypeIdxMap,
                         UnsignedMap);
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
        for (auto Arg = Fn.arg_begin(); Arg != Fn.arg_end(); ++Arg) {
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
        errs() << "Skipping fn " << DemangledName << "\n";
        continue;
      }
      errs() << "Instrumenting fn " << DemangledName << "\n";
      insertArgTracingHooks(m_module, Fn);
    }
  }

  bool finish() override { return true; }
};

Set<Str> collectTracedFunctionsForModule(Module &M) {
  Set<Str> result;

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
      errs() << "Skip empty\n";
      continue;
    }

    auto Pos = Data.find('\x00');
    if (Pos == Str::npos) {
      errs() << "functions-to-trace mapping: format invalid!\n";
      return Set<Str>();
    }

    auto ModId = Data.substr(0, Pos);
    if (ModId != M.getModuleIdentifier()) {
      errs() << "skip on module mismatch " << ModId << "\n";
      continue;
    }

    auto FnId = Data.substr(Pos + 1);
    errs() << "Add " << FnId << "\n";
    result.insert(FnId);
  }

  return result;
}

struct InsertFunctionCallPass : public PassInfoMixin<InsertFunctionCallPass> {

  PreservedAnalyses run(Module &M, ModuleAnalysisManager &AM) {
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
      errs() << "Instrumenting args...\n";
      Set<Str> TracedFns = collectTracedFunctionsForModule(M);
      ArgumentInstrumentation work(M, TracedFns);
      work.run();

      if (!work.finish()) {
        errs() << "Instrumentation failed - ArgumentInstrumentation!\n";
        exit(1);
      }
    } else {
      errs() << "Instrumenting fn entry...\n";
      FunctionEntryInsertion work(M);
      work.run();

      if (!work.finish()) {
        errs() << "Instrumentation failed - FunctionEntryInsertion!\n";
        errs() << "FunctionEntryInsertion requires -mllvm -llcap-mapdir DIR "
                  "directory!\n";
        exit(1);
      }
    }
    errs() << "Instrumentation DONE!\n";

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