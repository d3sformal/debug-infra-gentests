//===- Adapted from LLVM's PrintFunctionNames.cpp
//--------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/Decl.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendPluginRegistry.h"
#include "clang/Sema/Sema.h"
#include "llvm/ADT/StringRef.h"
#include <clang/AST/Decl.h>
#include <clang/Frontend/FrontendAction.h>
using namespace clang;

namespace {

llvm::StringRef KeyNormal = "VSTR-meta-location";
llvm::StringRef KeyTemplate = "VSTR-meta-location-template";

void addFunctionLocationMetadata(const FunctionDecl *FD,
                                 llvm::StringRef Key = KeyNormal) {
  constexpr llvm::StringRef metaLocYes = "y";
  constexpr llvm::StringRef metaLocNo = "n";

  auto &SourceManager = FD->getASTContext().getSourceManager();
  auto Loc = SourceManager.getExpansionLoc(FD->getBeginLoc());
  bool InSystemHeader = SourceManager.isInSystemHeader(Loc) ||
                        SourceManager.isInExternCSystemHeader(Loc) ||
                        SourceManager.isInSystemMacro(Loc);
  FD->setIrMetadata(Key, InSystemHeader ? metaLocNo : metaLocYes);
}

class AddMetadataConsumer : public ASTConsumer {
  CompilerInstance &Instance;
  std::set<std::string> ParsedTemplates;

public:
  AddMetadataConsumer(CompilerInstance &Instance,
                      std::set<std::string> ParsedTemplates)
      : Instance(Instance), ParsedTemplates(ParsedTemplates) {}

  bool HandleTopLevelDecl(DeclGroupRef DG) override {
    for (DeclGroupRef::iterator i = DG.begin(), e = DG.end(); i != e; ++i) {
      const Decl *D = *i;
      if (const FunctionDecl *FD = dyn_cast<FunctionDecl>(D)) {
        addFunctionLocationMetadata(FD);
      }
    }
    return true;
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
      addFunctionLocationMetadata(FD, KeyTemplate);
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
    return true;
  }
  ActionType getActionType() override { return AddBeforeMainAction; }
};
} // namespace

static FrontendPluginRegistry::Add<AddMetadataAction>
    X("ast-meta-add", "Propagates function location metadata from the AST");
