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
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/raw_ostream.h"
#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/Frontend/FrontendAction.h>

using namespace clang;

namespace {
  // TODO: remove the log debug info after finalization / decouple
void addFunctionLocationMetadata(const FunctionDecl *FD, bool log = false) {
  auto &SourceManager = FD->getASTContext().getSourceManager();
  auto Loc = SourceManager.getExpansionLoc(FD->getBeginLoc());
  bool InSystemHeader = SourceManager.isInSystemHeader(Loc) ||
                        SourceManager.isInExternCSystemHeader(Loc) ||
                        SourceManager.isInSystemMacro(Loc);
                        if (log) {
    llvm::outs() << FD->getDeclName() << ' ' << FD->getSourceRange().printToString(FD->getASTContext().getSourceManager()) << '\n';    
    FD->getNameForDiagnostic(llvm::outs(), PrintingPolicy(LangOptions()), true);
    llvm::outs() << '\n';
  }
  if (!InSystemHeader) {
    if (log) {
      llvm::outs() << "GOT "  "\n"; 
    }
    FD->setIrMetadata(VSTR_LLVM_NON_SYSTEMHEADER_FN_KEY, VSTR_LVVM_NON_SYSTEMHEADER_FN_VAL);
  } else {

    if (log) {   
      llvm::outs() << "NOT GOT\n" << SourceManager.isInSystemHeader(Loc) << " "
      << SourceManager.isInExternCSystemHeader(Loc) << " "
      << SourceManager.isInSystemMacro(Loc) << '\n'; 
    }
  }
}

class AddMetadataConsumer : public ASTConsumer {
public:
  AddMetadataConsumer() {}
  
  void HandleNamespaceDecl(const NamespaceDecl* ND) {
    for (auto It = ND->decls_begin(); It != ND->decls_end(); ++It) {
      Decl* D = *It;
      HandleDecl(D);
    }
  }

  void HandleDecl(Decl* D) {
      if (const FunctionDecl *FD = dyn_cast<FunctionDecl>(D)) {
        addFunctionLocationMetadata(FD);
      } else if (const NamespaceDecl* ND = dyn_cast<NamespaceDecl>(D)) {
        HandleNamespaceDecl(ND);
      }
      HandleAllLambdaExprsInDecl(D);
  }

  void HandleAllLambdaExprsInDecl(Decl *D) {

      // Handling of lambdas is different - lambdas are expressions => we have to
      // inspect the AST a bit more to get to the operator() of the anonymous type
      // that gets created for the closure
      struct LambdaVisitor : public RecursiveASTVisitor<LambdaVisitor> {     
        bool VisitLambdaExpr(const LambdaExpr * LE) {
          if (CXXMethodDecl* MD = LE->getCallOperator(); MD != nullptr) {
            addFunctionLocationMetadata(MD->getAsFunction());
          }
          return true;
        }
      } Lv;
      Lv.TraverseDecl(D);
  }

  bool HandleTopLevelDecl(DeclGroupRef DG) override {
    // TODO - HandleDecl is recursive via HandleNamespaceDecl -> if NamespaceDecls are NOT acyclic, we need to setup a set of visited/handled namespaces 
    for (DeclGroupRef::iterator i = DG.begin(), e = DG.end(); i != e; ++i) {
      Decl *D = *i;
      HandleDecl(D);
    }
    return true;
  }

  void HandleInlineFunctionDefinition(FunctionDecl* FD) override {
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
