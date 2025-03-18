#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Basic/LangOptions.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Rewrite/Core/Rewriter.h"
#include "clang/Tooling/CommonOptionsParser.h"
#include "clang/Tooling/Tooling.h"

#include "llvm/Support/CommandLine.h"
#include <cassert>
#include <iostream>
#include <utility>

using namespace clang::tooling;
using namespace llvm;
using namespace clang;

// Apply a custom category to all command-line options so that they are the
// only ones displayed.
static llvm::cl::OptionCategory MyToolCategory("my-tool options");

// CommonOptionsParser declares HelpMessage with a description of the common
// command-line options related to the compilation database and input files.
// It's nice to have this help message in all tools.
static cl::extrahelp CommonHelp(CommonOptionsParser::HelpMessage);

// A help message for this specific tool can be added afterwards.
static cl::extrahelp MoreHelp("\nMore help text...\n");

bool visitFunctionDecl(const FunctionDecl *Func, Rewriter &Rewriter) {
  bool Result = false;
  if (Func->hasBody()) {
    return Result;
  }
  for (unsigned int I = 0; I < Func->getNumParams(); I++) {
    std::string VarString = Func->parameters()[I]->getQualifiedNameAsString();
    Result = !Rewriter.InsertTextAfterToken(Func->getBody()->getBeginLoc(),
                                           "::__framework::Reporter::report(" +
                                               VarString + ", \"" + VarString +
                                               "\");\n");
  }
  return Result;
}

using namespace clang;
using namespace clang::ast_matchers;

DeclarationMatcher FunctionMatcher =
    functionDecl(anyOf(hasAnyParameter(hasType(asString("float"))),
                       hasAnyParameter(hasType(asString("int")))))
        .bind("functionDecl");

using RewDb = std::map<FileID, Rewriter>;

class FunctionDeclRewriter : public MatchFinder::MatchCallback {
  RewDb &MRewDb;

  Rewriter *getRewPtr(FileID Id, SourceManager *Mgr) {
    auto Found = MRewDb.find(Id);
    if (MRewDb.find(Id) != MRewDb.end()) {
      return &Found->second;
    }        
    MRewDb.insert(std::make_pair(Id, Rewriter(*Mgr, LangOptions())));
    return &MRewDb[Id];
  }

public:
  FunctionDeclRewriter(RewDb &RewDb) : MRewDb(RewDb) {}

  virtual void run(const MatchFinder::MatchResult &Result) override {
    assert(Result.SourceManager != nullptr);
    std::cout << "Trying!!\n";
    if (const FunctionDecl *FS =
            Result.Nodes.getNodeAs<clang::FunctionDecl>("functionDecl")) {
      // file id
      auto FileId = Result.SourceManager->getFileID(FS->getLocation());
      Rewriter *Rew = getRewPtr(FileId, Result.SourceManager);
      if (visitFunctionDecl(FS, *Rew)) {
        std::cout << "Modified " << FS->getNameAsString() << "\n";
        ;
      }
    }
  }
};

class Callbacks : public SourceFileCallbacks {

  RewDb &MRewriterDb;

public:
  Callbacks(RewDb &RewriterDb) : MRewriterDb(RewriterDb) {}

  void handleEndSource() override {
    std::cout << "End source \n";
    for (auto &&Rew : MRewriterDb) {
      if (Rew.second.overwriteChangedFiles()) {
        std::cout << "failed to flush \n";
      } else {
        std::cout << "flushed\n";
      }
    }
  }
};

int main(int argc, const char **argv) {
  auto ExpectedParser = CommonOptionsParser::create(argc, argv, MyToolCategory);
  if (!ExpectedParser) {
    // Fail gracefully for unsupported options.
    llvm::errs() << ExpectedParser.takeError();
    return 1;
  }
  CommonOptionsParser &OptionsParser = ExpectedParser.get();
  ClangTool Tool(OptionsParser.getCompilations(),
                 OptionsParser.getSourcePathList());

  RewDb Rewriters = {};
  Callbacks Db(Rewriters);

  FunctionDeclRewriter Rewriter(Rewriters);
  MatchFinder Finder;
  Finder.addMatcher(FunctionMatcher, &Rewriter);

  return Tool.run(newFrontendActionFactory(&Finder, &Db).get());
}