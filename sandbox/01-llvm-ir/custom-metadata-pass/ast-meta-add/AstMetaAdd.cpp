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
  CompilerInstance &Instance;
  std::set<std::string> ParsedTemplates;

public:
  AddMetadataConsumer(CompilerInstance &Instance,
                      std::set<std::string> ParsedTemplates)
      : Instance(Instance), ParsedTemplates(ParsedTemplates) {}
  
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

  void HandleTranslationUnit(ASTContext &context) override {
    if (!Instance.getLangOpts().DelayedTemplateParsing)
      return;

    // This demonstrates how to force instantiation of some templates in
    // -fdelayed-template-parsing mode. (Note: Doing this unconditionally for
    // all templates is similar to not using -fdelayed-template-parsig in the
    // first place.)
    // The advantage of doing this in HandleTranslationUnit() is that all
    // codegen (when using -add-plugin) is completely finished and this can't
    // affect the compiler output.
    struct Visitor : public RecursiveASTVisitor<Visitor> {
      const std::set<std::string> &ParsedTemplates;
      Visitor(const std::set<std::string> &ParsedTemplates)
          : ParsedTemplates(ParsedTemplates) {}
      bool VisitFunctionDecl(FunctionDecl *FD) {
        if (FD->isLateTemplateParsed() &&
            ParsedTemplates.count(FD->getNameAsString()))
          LateParsedDecls.insert(FD);
        return true;
      }

      std::set<FunctionDecl *> LateParsedDecls;
    } v(ParsedTemplates);

    v.TraverseDecl(context.getTranslationUnitDecl());

    clang::Sema &sema = Instance.getSema();
    for (const FunctionDecl *FD : v.LateParsedDecls) {
      clang::LateParsedTemplate &LPT =
          *sema.LateParsedTemplateMap.find(FD)->second;
      sema.LateTemplateParser(sema.OpaqueParser, LPT);
      addFunctionLocationMetadata(FD);
    }
  }
};

class AddMetadataAction : public PluginASTAction {
  std::set<std::string> ParsedTemplates;

protected:
  std::unique_ptr<ASTConsumer> CreateASTConsumer(CompilerInstance &CI,
                                                 llvm::StringRef) override {
    return std::make_unique<AddMetadataConsumer>(CI, ParsedTemplates);
  }

  bool ParseArgs(const CompilerInstance &CI,
                 const std::vector<std::string> &args) override {
    // TODO: library function filter via option (mangled name vs metadata)
    return true;
  }
  ActionType getActionType() override { return AddBeforeMainAction; }
};
} // namespace

static FrontendPluginRegistry::Add<AddMetadataAction>
    X("ast-meta-add", "Propagates function location metadata from the AST");
