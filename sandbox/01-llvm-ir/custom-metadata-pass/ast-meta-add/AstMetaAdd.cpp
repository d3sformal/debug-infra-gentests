//===- Adapted from LLVM's PrintFunctionNames.cpp
//--------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
#include "./llvm-metadata.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/PrettyPrinter.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Basic/LangOptions.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendPluginRegistry.h"
#include "clang/Sema/Sema.h"
#include "llvm/ADT/StringExtras.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/raw_ostream.h"
#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/Frontend/FrontendAction.h>
#include <sstream>
#include <string>
#include <vector>

using namespace clang;

namespace {

// used with StringRefs - they do not own data they reference - we must ensure
// the lifetime of our metadata strings survives up until the IR generation
static std::set<std::string> StringBackings;

bool isTargetTypeValRefPtr(const std::string &s, const std::string &tgt) {
  return s == tgt || s == tgt + " *" || s == tgt + " &" || s == tgt + " &&";
}

// Predicate :: (ParmVarDecl* param, size_t ParamIndex) -> bool
template <typename Predicate>
std::vector<size_t> filterParmIndicies(const FunctionDecl *FD, Predicate pred) {
  std::vector<size_t> Indicies;
  size_t ParamIndex = 0;
  for (ParmVarDecl *const *it = FD->param_begin(); it != FD->param_end();
       ++it) {
    auto *param = *it;
    if (pred(param, ParamIndex)) {
      Indicies.push_back(ParamIndex);
    }
    ++ParamIndex;
  }
  return Indicies;
}

void addIndiciesMetadata(const llvm::StringRef MetaKey, const FunctionDecl *FD,
                         const std::vector<size_t> &Indicies) {
  std::stringstream ResStream("");
  for (size_t i = 0; i < Indicies.size(); ++i) {
    ResStream << std::to_string(Indicies[i]);
    if (i != Indicies.size() - 1) {
      ResStream << VSTR_LLVM_CXX_SINGLECHAR_SEP;
    }
  }
  // all this could be theoretically done in a much more lightweight fashion
  // using metadata with multiple numeric operands (but I have not yet exposed
  // this through the patched API)
  auto Res = ResStream.str();
  if (Res.length() > 0) {
    StringBackings.emplace(Res);
    FD->setIrMetadata(MetaKey, *StringBackings.find(Res));
  }
}

// Predicate :: (ParmVarDecl* param, size_t ParamIndex) -> bool
template <typename Predicate>
void encodeArgIndiciesSatisfying(const StringRef MetadataKey,
                                 const FunctionDecl *FD, Predicate Pred) {
  auto Indicies = filterParmIndicies(FD, Pred);
  addIndiciesMetadata(MetadataKey, FD, Indicies);
}

void addFunctionLocationMetadata(const FunctionDecl *FD, bool log = false) {
  auto &SourceManager = FD->getASTContext().getSourceManager();
  auto Loc = SourceManager.getExpansionLoc(FD->getBeginLoc());
  bool InSystemHeader = SourceManager.isInSystemHeader(Loc) ||
                        SourceManager.isInExternCSystemHeader(Loc) ||
                        SourceManager.isInSystemMacro(Loc);
  if (log) {
    llvm::errs() << FD->getDeclName() << ' '
                 << FD->getSourceRange().printToString(
                        FD->getASTContext().getSourceManager())
                 << '\n';
    FD->getNameForDiagnostic(llvm::errs(), PrintingPolicy(LangOptions()), true);
    llvm::errs() << '\n';
  }
  if (!InSystemHeader) {
    if (log) {
      llvm::errs() << "GOT "
                      "\n";
    }

    encodeArgIndiciesSatisfying(
        VSTR_LLVM_CXX_DUMP_STDSTRING, FD, [](ParmVarDecl *Arg, size_t Idx) {
          auto TypeName = Arg->getType().getCanonicalType().getAsString();
          return isTargetTypeValRefPtr(TypeName,
                                       "class std::basic_string<char>");
        });

    encodeArgIndiciesSatisfying(
        VSTR_LLVM_UNSIGNED_IDCS, FD, [](ParmVarDecl *Arg, size_t Idx) {
          return Arg->getType()->isUnsignedIntegerType();
        });

    FD->setIrMetadata(VSTR_LLVM_NON_SYSTEMHEADER_FN_KEY,
                      VSTR_LVVM_NON_SYSTEMHEADER_FN_VAL);
    if (FD->isCXXInstanceMember()) {
      FD->setIrMetadata(VSTR_LLVM_CXX_THISPTR, "");
    }
  } else {

    if (log) {
      llvm::errs() << "NOT GOT\n"
                   << SourceManager.isInSystemHeader(Loc) << " "
                   << SourceManager.isInExternCSystemHeader(Loc) << " "
                   << SourceManager.isInSystemMacro(Loc) << '\n';
    }
  }
}

class AddMetadataConsumer : public ASTConsumer {
public:
  AddMetadataConsumer() {}

  void HandleNamespaceDecl(const NamespaceDecl *ND) {
    for (auto It = ND->decls_begin(); It != ND->decls_end(); ++It) {
      Decl *D = *It;
      HandleDecl(D);
    }
  }

  void HandleDecl(Decl *D) {
    if (const FunctionDecl *FD = dyn_cast<FunctionDecl>(D)) {
      addFunctionLocationMetadata(FD);
    } else if (const NamespaceDecl *ND = dyn_cast<NamespaceDecl>(D)) {
      HandleNamespaceDecl(ND);
    }
    HandleAllLambdaExprsInDecl(D);
  }

  void HandleAllLambdaExprsInDecl(Decl *D) {

    // Handling of lambdas is different - lambdas are expressions => we have to
    // inspect the AST a bit more to get to the operator() of the anonymous type
    // that gets created for the closure
    struct LambdaVisitor : public RecursiveASTVisitor<LambdaVisitor> {
      bool VisitLambdaExpr(const LambdaExpr *LE) {
        if (CXXMethodDecl *MD = LE->getCallOperator(); MD != nullptr) {
          addFunctionLocationMetadata(MD->getAsFunction());
        }
        return true;
      }
    } Lv;
    Lv.TraverseDecl(D);
  }

  bool HandleTopLevelDecl(DeclGroupRef DG) override {
    // TODO - HandleDecl is recursive via HandleNamespaceDecl -> if
    // NamespaceDecls are NOT acyclic, we need to setup a set of visited/handled
    // namespaces
    for (DeclGroupRef::iterator i = DG.begin(), e = DG.end(); i != e; ++i) {
      Decl *D = *i;
      HandleDecl(D);
    }
    return true;
  }

  void HandleInlineFunctionDefinition(FunctionDecl *FD) override {
    addFunctionLocationMetadata(FD);
  }
};

class AddMetadataAction : public PluginASTAction {
protected:
  std::unique_ptr<ASTConsumer> CreateASTConsumer(CompilerInstance &CI,
                                                 llvm::StringRef) override {
    return std::make_unique<AddMetadataConsumer>();
  }

  bool ParseArgs(const CompilerInstance &CI,
                 const std::vector<std::string> &args) override {
    return true;
  }

  ActionType getActionType() override { return AddBeforeMainAction; }
};
} // namespace

static FrontendPluginRegistry::Add<AddMetadataAction>
    X("ast-meta-add", "Propagates function location metadata from the AST");
