#include "instrumentation.hpp"

#include "../../custom-metadata-pass/ast-meta-add/llvm-metadata.h"
#include "argMapping.hpp"
#include "constants.hpp"
#include "modMapping.hpp"
#include "typeAlias.hpp"
#include "typeids.h"
#include "utility.hpp"
#include "llvm/ADT/APInt.h"
#include "llvm/ADT/SmallVector.h"
#include <llvm/ADT/StringRef.h>
#include <llvm/Analysis/TargetLibraryInfo.h>
#include <llvm/Demangle/Demangle.h>
#include "llvm/IR/BasicBlock.h"
#include <llvm/IR/Constants.h>
#include <llvm/IR/DerivedTypes.h>
#include "llvm/IR/Function.h"
#include <llvm/IR/GlobalVariable.h>
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/InstIterator.h"
#include "llvm/IR/Instruction.h"
#include <llvm/IR/LLVMContext.h>
#include "llvm/IR/Metadata.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/PassManager.h"
#include <llvm/IR/Type.h>
#include "llvm/Pass.h"
#include <llvm/Passes/OptimizationLevel.h>
#include "llvm/Passes/PassBuilder.h"
#include "llvm/Support/Alignment.h"
#include "llvm/Support/Casting.h"
#include "llvm/Support/raw_ostream.h"
#include <array>
#include <cstdint>
#include <fstream>
#include <ios>
#include <string>
#include <type_traits>
#include <unordered_map>
#include <utility>

using namespace llvm;

// functions common to both instrumentations
namespace common {
namespace {

// data helping to implement custom type support

struct SCustomTypeDescription {
  // the exact name of the hook as available in the hooklib
  const char *m_hookFnName;
  // display name that may appear in log entries
  const char *m_log_name;
};

// maps metadata key (correspoding to a custom type) to the size of the type
// argument, for custom types, LLSZ_CUSTOM is the only one valid at this point
// and instrumentation is done via pointer/reference
const std::unordered_map<const char *, LlcapSizeType> SCustomSizes{
    {LLCAP_TYPE_STD_STRING, LlcapSizeType::LLSZ_CUSTOM},
    // invalid size means
    // that this type index is just a "flag" and
    // has no effect on the "real argument size" that the instrumentation will
    // work with
    {LLCAP_UNSIGNED_IDCS, LlcapSizeType::LLSZ_INVALID}};

const std::unordered_map<const char *, SCustomTypeDescription> SCustomHooks{
    {LLCAP_TYPE_STD_STRING,
     SCustomTypeDescription{.m_hookFnName = "llcap_hooklib_extra_cxx_string",
                            .m_log_name = "std::string"}}};

// creates argument index mapping for a particular function, taking into account
// all of the above-registered custom type metadata keys
ClangMetadataToLLVMArgumentMapping
createArgumentMapping(Function &Fn, IdxMappingInfo &IdxInfo) {
  ClangMetadataToLLVMArgumentMapping Mapping(Fn, IdxInfo);
  for (auto &&[key, size] : SCustomSizes) {
    Mapping.registerCustomTypeIndicies(key, size);
  }
  return Mapping;
}

// helper container for IR-level constants
// these are used in calls to hooklib functions which accept module and function
// IDs (the IDs are inserted during the instrumentation as constants)
struct SFnUidConstants {
  ConstantInt *module;
  ConstantInt *function;

  // creates the constant pair inside the supplied module
  static SFnUidConstants getModFunIdConstants(llcap::ModuleId ModuleIntId,
                                              Module &M,
                                              llcap::FunctionId FunctionIntId) {
    static_assert(sizeof(llcap::FunctionId) ==
                  4); // this does not imply incorrectness, just that everything
    // must be checked
    auto *FnIdConstant = ConstantInt::get(
        M.getContext(), APInt(llcap::FUNID_BITSIZE, FunctionIntId));
    static_assert(sizeof(llcap::ModuleId) == 4);
    auto *ModIdConstant = ConstantInt::get(
        M.getContext(), APInt(llcap::MODID_BITSIZE, ModuleIntId));
    return {.module = ModIdConstant, .function = FnIdConstant};
  }
};

// inserts a call to the string-specified function
// and supplies the Module and Function ID to it (in this order)
void insertInfraFnCall(IRBuilder<> &Builder, Module &M, StringRef Name,
                       const common::SFnUidConstants C) {
  auto Callee = M.getOrInsertFunction(
      Name,
      FunctionType::get(Type::getVoidTy(M.getContext()),
                        {C.module->getType(), C.function->getType()}, false));
  Builder.CreateCall(Callee, {C.module, C.function});
}
} // namespace
} // namespace common

// functions relevant only to the call tracing instrumentation
namespace callTracing {
namespace {

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

// decides whether the function is "in a system header" by inspecting metadata
// and looking for keys inserted by the AST pass
bool isStdFnBasedOnMetadata(const Function &Fn,
                            const std::string &DemangledName,
                            const StringRef MangledName) {
  VERBOSE_LOG << "Metadata of function " << DemangledName << '\n';
  if (MDNode *N = Fn.getMetadata(LLCAP_FN_NOT_IN_SYS_HEADER_KEY)) {
    if (N->getNumOperands() == 0) {
      VERBOSE_LOG << "Warning! Unexpected metadata node with no "
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

// decides whether the function is "in a system header"
// we do not instrument such functions
bool isStdFn(const Function &Fn, const Str &DemangledName, const StringRef Name,
             bool UseMangledNames) {
  if (UseMangledNames && isStdFnDanger(Name)) {
    return true;
  }
  if (!UseMangledNames && isStdFnBasedOnMetadata(Fn, DemangledName, Name)) {
    return true;
  }
  return false;
}

void insertFnEntryHook(IRBuilder<> &Builder, Module &M,
                       const common::SFnUidConstants C) {
  common::insertInfraFnCall(Builder, M, "hook_start", C);
}
} // namespace
} // namespace callTracing

namespace argCapture {

// inserts test-terminating call to hooklib before every potentially-exitting IR
// instruction this includes exception-related instructions
//
// WARNING: exceptions are only partially covered
void insertTestEpilogueHook(Function &Fn, Module &M,
                            const common::SFnUidConstants C) {
  auto Types = {C.module->getType(), C.function->getType()};
  FunctionCallee EpilogueCallFn = M.getOrInsertFunction(
      "hook_test_epilogue",
      FunctionType::get(Type::getVoidTy(M.getContext()), Types, false));

  FunctionCallee EpilogueExceptionFn = M.getOrInsertFunction(
      "hook_test_epilogue_exc",
      FunctionType::get(Type::getVoidTy(M.getContext()), Types, false));
  // we need to walk all the basic blocks, look for ret, resume, catchswitch,
  // cleanupret instructions and place a call before them

  // all the crappery below simply modifies the instructions ret, resume,
  // catchswitch, cleanupret by INSERTING A CALL BEFORE those instructions

  // it looks ugly due to various method deprecations & mainly iterator
  // invalidation (inst_iterator) - with each modified instruction, we must
  // re-iterate the instructions (hence the while true)

  // so far, I haven't found a better, correct way to do this. Furthermore,
  // we mark "how many instructions should we skip to
  // get back to the place we left off" to make this scan linear
  // (ToSkip, the small for-loop)
  u32 ToSkip = 0;
  while (true) {
    inst_iterator I = inst_begin(Fn);
    inst_iterator E = inst_end(Fn);
    // skip instructions to get to the last "instrumented" instruction
    // (i cannot take the difference between I and E to make this
    // straightforward, via I += std::min(ToSkip, E - I) and using std::distance
    // returns an int which cannot be added to the inst_iterator...)
    for (u32 Skip = 0; Skip < ToSkip && I != E; ++Skip) {
      ++I;
    }

    if (I == E) {
      break;
    }

    for (; I != E; ++I) {
      // increment skip offset
      ++ToSkip;
      if (I->getOpcode() == Instruction::Ret ||
          I->getOpcode() == Instruction::Resume) {
        CallInst *CallInsn = CallInst::Create(
            (I->getOpcode() == Instruction::Resume ? EpilogueExceptionFn
                                                   : EpilogueCallFn),
            {C.module, C.function});
        CallInsn->insertBefore(I->getIterator());
        // add an instruction to skip -> we should skip past the Ret/Resume
        ++ToSkip;
        // iterators are invalidated, we must loop again
        break;
      }

      if (I->getOpcode() == Instruction::CatchSwitch) {
        outs()
            << "CatchSwitch instruction encountered, this is unhandled yet!\n";
        continue;
      }

      if (I->getOpcode() == Instruction::CleanupRet) {
        outs()
            << "CleanupRet instruction encountered, this is unhandled yet!\n";
        continue;
      }
    }
  }
}

void insertArgCapturePreambleHook(IRBuilder<> &Builder, Module &M,
                                  const common::SFnUidConstants &C) {
  common::insertInfraFnCall(Builder, M, "hook_arg_preamble", C);
}

void instrumentArgHijack(IRBuilder<> &Builder, Module &M, Argument *Arg,
                         Type *Ty, const FunctionCallee &Callee,
                         ConstantInt *ModId, ConstantInt *FnId) {
  // inserts alloca, call, load instruction sequence where
  // the alloca allocates "some" bytes, pointer to those bytes
  // is passed to a hooklib call (along with original argument)
  // and load subsequently loads from the alloca'd address
  //
  // it is expected hooklib somehow initializes the pointer to newly
  // allocated data

  // Weirdness introduced by argument hijacking:
  // - destructors (where to call, for what object) - not called (const-ness,
  // similarly why value/property replacement in-place is not performed)
  // - data passed in more than one register (Arg would only be half of
  // the data e.g. for 128bit number) - this should be handled correctly by
  // hijacking all parts of such arguments hopefully (current index shifting is
  // done based only on sret). Plus, custom data shall be only instrumented
  // by-pointer, not by value
  auto *Alloca = Builder.CreateAlloca(Ty);

  if (Alloca == nullptr) {
    errs() << "Instrumentation failed: Alloca\n";
    exit(1);
  }
  IF_DEBUG {
    Alloca->dump();
    errs() << "OPERAND count " << Alloca->getNumOperands() << '\n';
    errs() << "OPERAND " << Alloca->getNameOrAsOperand() << " DUMP\n";
  }
  auto *Op = Alloca->getOperand(0);
  IF_DEBUG Op->dump();

  auto *Call = Builder.CreateCall(Callee, {Arg, Alloca, ModId, FnId});

  auto *Load = Builder.CreateAlignedLoad(
      Ty, Alloca, M.getDataLayout().getPrefTypeAlign(Ty));
  if (Alloca == nullptr) {
    errs() << "Instrumentation failed: Load\n";
    exit(1);
  }

  // replaces all usages Arg (argument to be captured/hijacked)
  // with the newly loaded value
  Vec<llvm::Use *> ArgUsages;
  for (auto Use = Arg->use_begin(); Use != Arg->use_end(); ++Use) {
    // do not replace the usage inside our own call instruction
    if (Use->getUser() == Call) {
      continue;
    }

    ArgUsages.push_back(&*Use);
  }

  for (auto *ArgUse : ArgUsages) {
    IF_DEBUG {
      errs() << "For use in" << '\n';
      ArgUse->getUser()->dump();
      errs() << "Setting arg no " << ArgUse->getOperandNo() << " to new load\n";
    }
    ArgUse->set(Load);
  }
}

FunctionCallee getOrInsertHookFn(const char *HookName, Type *TypePtr, Module &M,
                                 LLVMContext &Ctx) {
  return M.getOrInsertFunction(
      HookName,
      FunctionType::get(Type::getVoidTy(Ctx),
                        {TypePtr, PointerType::getUnqual(Ctx),
                         Type::getInt32Ty(Ctx), Type::getInt32Ty(Ctx)},
                        false));
}

// returns false if argument instrumentation shoudl attempt different types of
// arguments
bool tryInsertIntegerArgCapture(
    IRBuilder<> &Builder, LLVMContext &Ctx, Module &M, unsigned int ArgNum,
    Type *ArgT, const common::SFnUidConstants &C, Argument *Arg,
    const ClangMetadataToLLVMArgumentMapping &Mapping,
    const Vec<Pair<size_t, LlcapSizeType>> &Sizes) {
  auto GetOrInsertHookFn = [&](const char *HookName, Type *TypePtr) {
    return getOrInsertHookFn(HookName, TypePtr, M, Ctx);
  };
  auto CallByte = GetOrInsertHookFn("hook_char", Type::getInt8Ty(Ctx));
  auto CallUnsByte = GetOrInsertHookFn("hook_uchar", Type::getInt8Ty(Ctx));
  auto CallShort = GetOrInsertHookFn("hook_short", Type::getInt8Ty(Ctx));
  auto CallUnsShort = GetOrInsertHookFn("hook_ushort", Type::getInt8Ty(Ctx));
  auto CallInt32 = GetOrInsertHookFn("hook_int32", Type::getInt32Ty(Ctx));
  auto CallUnsInt32 = GetOrInsertHookFn("hook_uint32", Type::getInt32Ty(Ctx));
  auto CallInt64 = GetOrInsertHookFn("hook_int64", Type::getInt64Ty(Ctx));
  auto CallUnsInt64 = GetOrInsertHookFn("hook_uint64", Type::getInt64Ty(Ctx));

  const Map<LlcapSizeType, Tuple<FunctionCallee, FunctionCallee, Type *>>
      IntTypeSizeMap = {{LlcapSizeType::LLSZ_8,
                         Tuple{CallUnsByte, CallByte, Type::getInt8Ty(Ctx)}},
                        {LlcapSizeType::LLSZ_16,
                         Tuple{CallUnsShort, CallShort, Type::getInt16Ty(Ctx)}},
                        {LlcapSizeType::LLSZ_32,
                         Tuple{CallUnsInt32, CallInt32, Type::getInt32Ty(Ctx)}},
                        {LlcapSizeType::LLSZ_64, Tuple{CallUnsInt64, CallInt64,
                                                       Type::getInt64Ty(Ctx)}}};
  static_assert(std::underlying_type_t<LlcapSizeType>(LlcapSizeType::LLSZ_8) ==
                    1,
                "Failed basic check");

  auto IsAttrUnsgined = Mapping.llvmArgNoMatches(ArgNum, LLCAP_UNSIGNED_IDCS);
  auto ThisArgSize = Sizes[ArgNum].second;
  if (!isValid(ThisArgSize)) {
    errs()
        << "Encountered an invalid argument size specifier, cannot instrument";
    IF_VERBOSE {
      errs() << " arg:\n";
      Arg->dump();
    }
    errs() << "\n";
    return true;
  }

  if (IntTypeSizeMap.contains(ThisArgSize)) {
    const unsigned int Bits =
        std::underlying_type_t<LlcapSizeType>(ThisArgSize) * 8;
    if (ArgT->isIntegerTy(Bits)) {
      auto &&[UnsFn, SignFn, TPtr] = IntTypeSizeMap.at(ThisArgSize);
      VERBOSE_LOG << "Inserting call " << std::to_string(Bits)
                  << (IsAttrUnsgined ? "U\n" : "S\n");
      instrumentArgHijack(Builder, M, Arg, TPtr,
                          IsAttrUnsgined ? UnsFn : SignFn, C.module,
                          C.function);
      return true;
    }
  }
  return false;
}

// terminology:
// LLVM Argument Index = 0-based index of an argument as seen directly in the
// LLVM IR

//  AST Argument Index  = 0-based idx as seen in the Clang AST

// key differences accounted for: this pointer & sret arguments (returning a
// struct in a register)

void insertArgCaptureHook(IRBuilder<> &Builder, Module &M,
                          const common::SFnUidConstants &C, Argument *Arg,
                          const ClangMetadataToLLVMArgumentMapping &Mapping,
                          const Vec<Pair<size_t, LlcapSizeType>> &Sizes) {
  auto &Ctx = M.getContext();
  auto GetOrInsertHookFn = [&](const char *HookName, Type *TypePtr) {
    return getOrInsertHookFn(HookName, TypePtr, M, Ctx);
  };

  auto ArgNum = Arg->getArgNo();
  auto *ArgT = Arg->getType();

  // we first attempt to insert floating point type's instrumentation
  // then integers,
  // then custom types

  // NOTE: the fine-grained branching on the LLVM type is not necessary
  // and was left behind in case it is needed, reducing hooks to 1, 2, 4 and
  // 8-byte hooks inspecting only the size of the argument should be enough to
  // perform everything in the curent implentation of the workflow

  if (ArgT->isFloatTy()) {
    VERBOSE_LOG << "Inserting call f32\n";
    auto *TPtr = Type::getFloatTy(Ctx);
    auto CallFloat = GetOrInsertHookFn("hook_float", TPtr);
    instrumentArgHijack(Builder, M, Arg, TPtr, CallFloat, C.module, C.function);
    return;
  }

  if (ArgT->isDoubleTy()) {
    VERBOSE_LOG << "Inserting call f64\n";
    auto *TPtr = Type::getDoubleTy(Ctx);
    auto CallDouble = GetOrInsertHookFn("hook_double", TPtr);
    instrumentArgHijack(Builder, M, Arg, TPtr, CallDouble, C.module,
                        C.function);
    return;
  }

  if (tryInsertIntegerArgCapture(Builder, Ctx, M, ArgNum, ArgT, C, Arg, Mapping,
                                 Sizes)) {
    return;
  }

  bool CustomTypeInstrumented = false;
  for (auto &&[key, desc] : common::SCustomHooks) {
    if (Mapping.llvmArgNoMatches(ArgNum, key)) {
      if (!ArgT->isPointerTy()) {
        errs() << desc.m_log_name
               << " hooks cannot handle non-pointer argument of this "
                  "type yet\n";
        return;
      }

      VERBOSE_LOG << "Inserting call " << desc.m_log_name << "\n";
      auto CallCxxString = GetOrInsertHookFn(desc.m_hookFnName, ArgT);
      instrumentArgHijack(Builder, M, Arg, ArgT, CallCxxString, C.module,
                          C.function);
      CustomTypeInstrumented = true;
      break;
    }
  }

  if (CustomTypeInstrumented) {
    return;
  }

  IF_VERBOSE errs() << "Encountered an unknown argument size specifier "
                    << std::underlying_type_t<LlcapSizeType>(
                           Sizes[ArgNum].second)
                    << '\n';
  IF_VERBOSE Arg->dump();
}

// parses the function selection file
// returns the module id and the function name -> function ID mapping
Maybe<Pair<llcap::ModuleId, Map<Str, uint32_t>>>
collectTracedFunctionsForModule(Module &M, const Str &SelectionPath) {
  Map<Str, uint32_t> Map;
  Maybe<llcap::ModuleId> NumericModId = NONE;

  if (SelectionPath.empty()) {
    return NONE;
  }

  std::ifstream Targets(SelectionPath, std::ios::binary);
  if (!Targets) {
    errs() << "Could not open targets file @ " << SelectionPath << "\n";
    return NONE;
  }

  // finds the next 0x00 after Pos in Data, writes the found position to NextPos
  // returns true if NextPos is valid
  auto NextPosMove = [](const Str &Data, const u64 &Pos, auto &NextPos,
                        const char *Msg) {
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
  // this loop implements split by `0x0` for each line of the selection file
  // and passes the items of the split into string, number, string and a number
  // corresponding to the module name, id, function name id
  while (std::getline(Targets, Data, '\n')) {
    if (Data.empty()) {
      DEBUG_LOG << "Skip empty\n";
      Pos = 0;
      continue;
    }

    u64 NextPos = Pos; // Module name
    if (!NextPosMove(Data, Pos, NextPos, "mod id")) {
      return NONE;
    }

    Str ModId = Data.substr(Pos, NextPos - Pos);
    if (ModId != M.getModuleIdentifier()) {
      DEBUG_LOG << "Skip on module mismatch " << ModId << "\n";
      Pos = 0;
      continue;
    }
    // incrementing the next index to skip the 0x00
    Pos = NextPos + 1;

    // module numerical id
    if (!NextPosMove(Data, Pos, NextPos, "mod id n")) {
      return NONE;
    }
    Maybe<u64> ModIdRes =
        tryParse<llcap::ModuleId>(Data.substr(Pos, NextPos - Pos));
    if (!ModIdRes) {
      DEBUG_LOG << "functions-to-trace mapping: format invalid (mod id n)\n";
      return NONE;
    }
    NumericModId = ModIdRes;
    Pos = NextPos + 1;

    // Function name
    if (!NextPosMove(Data, Pos, NextPos, "fn id")) {
      return NONE;
    }
    auto FnId = Data.substr(Pos, NextPos - Pos);
    Pos = NextPos + 1;

    // the rest is expected to be the
    // Function numeric ID
    auto FnIdNumeric = tryParse<llcap::FunctionId>(Data.substr(Pos));
    if (!FnIdNumeric) {
      DEBUG_LOG << "functions-to-trace mapping: format invalid (fn id n)\n";
      return NONE;
    }

    VERBOSE_LOG << "Add \"to trace\" " << FnId << ", ID: " << *FnIdNumeric
                << "\n";
    Map[FnId] = *FnIdNumeric;
    Pos = 0;
  }

  if (!NumericModId) {
    return NONE;
  }

  return std::make_pair(*NumericModId, Map);
}

} // namespace argCapture

Instrumentation::Instrumentation(llvm::Module &M,
                                 std::shared_ptr<const Config> Cfg)
    : m_module(M), m_cfg(std::move(Cfg)) {
  if (auto MbInfo = IdxMappingInfo::parseFromModule(m_module); MbInfo) {
    m_idxInfo = *MbInfo;
    m_skip = false;
  } else {
    // not really sure if "all" collides with other modules or not? => remain
    // pessimistic
    m_skip = true;
  }
}

void FunctionEntryInstrumentation::instrument() {
  if (m_skip) {
    VERBOSE_LOG << "Skipping entire module " + m_module.getModuleIdentifier()
                << '\n';
    return;
  }
  if (!m_ready) {
    VERBOSE_LOG << "Instrumentation not ready, module " +
                       m_module.getModuleIdentifier()
                << '\n';
    exit(1);
  }

  for (Function &Fn : m_module) {
    // Skip library functions
    StringRef MangledName = Fn.getFunction().getName();
    Str DemangledName = llvm::demangle(MangledName);

    if (callTracing::isStdFn(Fn, DemangledName, MangledName,
                             m_cfg->useMangledNames)) {
      continue;
    }

    BasicBlock &EntryBB = Fn.getEntryBlock();
    IRBuilder<> Builder(&EntryBB.front());

    // Note: there are more LLVM IR types that theoretically could be handled
    // in the future (e.g. the SIMD Vector type)
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

    ClangMetadataToLLVMArgumentMapping Mapping =
        common::createArgumentMapping(Fn, m_idxInfo);
    const auto FunId = m_fnIdMap.addFunction(DemangledName, Mapping);

    auto Constants = common::SFnUidConstants::getModFunIdConstants(
        m_fnIdMap.getModuleMapIntId(), m_module, FunId);

    callTracing::insertFnEntryHook(Builder, m_module, Constants);
  }
}

bool FunctionEntryInstrumentation::finish() {
  auto ModMapsDir =
      m_cfg->modMapsDir.empty() ? "module-maps" : m_cfg->modMapsDir;
  return FunctionIDMapper::flush(std::move(m_fnIdMap), ModMapsDir);
}

ArgumentInstrumentation::ArgumentInstrumentation(
    llvm::Module &M, std::shared_ptr<const Config> Cfg)
    : Instrumentation(M, std::move(Cfg)) {

  auto TracedFns =
      argCapture::collectTracedFunctionsForModule(M, m_cfg->SelectionPath);
  if (!TracedFns) {
    m_ready = false;
  } else {
    m_moduleId = TracedFns->first;
    m_traced_functions = std::move(TracedFns->second);
    m_ready = true;
  }
}

void ArgumentInstrumentation::instrument() {
  if (m_skip) {
    VERBOSE_LOG << "Skipping entire module " + m_module.getModuleIdentifier()
                << '\n';
    return;
  }
  if (!m_ready) {
    VERBOSE_LOG << "Instrumentation not ready, module " +
                       m_module.getModuleIdentifier()
                << '\n';
    exit(1);
  }

  for (Function &Fn : m_module) {
    StringRef MangledName = Fn.getFunction().getName();
    Str DemangledName = llvm::demangle(MangledName);

    auto FnId = m_traced_functions.find(DemangledName);
    if (FnId == m_traced_functions.end()) {
      DEBUG_LOG << "Skipping fn " << DemangledName << "\n";
      continue;
    }
    VERBOSE_LOG << "Instrumenting fn " << DemangledName << "\n";

    BasicBlock &EntryBB = Fn.getEntryBlock();
    IRBuilder<> Builder(&EntryBB.front());
    auto Constants = common::SFnUidConstants::getModFunIdConstants(
        m_moduleId, m_module, FnId->second);

    ClangMetadataToLLVMArgumentMapping Mapping =
        common::createArgumentMapping(Fn, m_idxInfo);
    argCapture::insertArgCapturePreambleHook(Builder, m_module, Constants);

    for (auto *Arg = Fn.arg_begin(); Arg != Fn.arg_end(); ++Arg) {
      argCapture::insertArgCaptureHook(Builder, m_module, Constants, Arg,
                                       Mapping, Mapping.getArgumentSizeTypes());
    }

    if (m_cfg->performFnExitInstrumentation) {
      argCapture::insertTestEpilogueHook(Fn, m_module, Constants);
    }
  }
}