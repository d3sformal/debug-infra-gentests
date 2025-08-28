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
#include "llvm/Support/raw_ostream.h"
#include <cassert>
#include <utility>
#include <set>
#include <fstream>

using namespace clang::tooling;
using namespace llvm;
using namespace clang;

// Apply a custom category to all command-line options so that they are the
// only ones displayed.
static llvm::cl::OptionCategory MyToolCategory("Tool options");

static cl::opt<bool> ProduceFileList("F",cl::desc("Produce a list of modified files"),
cl::cat(MyToolCategory));

// CommonOptionsParser declares HelpMessage with a description of the common
// command-line options related to the compilation database and input files.
// It's nice to have this help message in all tools.
static cl::extrahelp CommonHelp(CommonOptionsParser::HelpMessage);

// A help message for this specific tool can be added afterwards.
static cl::extrahelp MoreHelp("\nMore help text...\n");

bool visitFunctionDecl(const FunctionDecl *Func, Rewriter &Rewriter) {
  bool Result = false;
  llvm::outs() << "Visiting " << Func->getQualifiedNameAsString() << '\n';
  if (!Func->hasBody()) {
    llvm::errs() << "\tno body\n";
    return Result;
  }
  llvm::outs() << "\tParameters\n";
  for (unsigned int I = 0; I < Func->getNumParams(); I++) {
    std::string VarString = Func->parameters()[I]->getQualifiedNameAsString();
    llvm::outs() << "\t\t" << VarString  << "\n";
    if (Func->getBody() == nullptr) {
      continue;
    }
    Result = !Rewriter.InsertTextAfterToken(Func->getBody()->getBeginLoc(),
                                           "\n::__framework::Reporter::report(" +
                                               VarString + ", \"" + VarString +
                                               "\");\n");
  }
  return Result;
}

using namespace clang;
using namespace clang::ast_matchers;

DeclarationMatcher FunctionMatcher =
    functionDecl(allOf(isExpansionInMainFile(), anyOf(hasAnyParameter(hasType(asString("float"))),
                       hasAnyParameter(hasType(asString("int"))))))
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

  std::set<std::string> MFileNames;
  bool MCollectFiles{false};
public:
  FunctionDeclRewriter(RewDb &RewDb, bool CollectFiles = false) : MRewDb(RewDb), MCollectFiles(CollectFiles) {}

  virtual void run(const MatchFinder::MatchResult &Result) override {
    assert(Result.SourceManager != nullptr);
    if (const FunctionDecl *FS =
            Result.Nodes.getNodeAs<clang::FunctionDecl>("functionDecl")) {
      // file id
      auto* SrcMgr = Result.SourceManager;
      auto FileId = SrcMgr->getFileID(FS->getLocation());
      llvm::outs() << "Trying file: [" << FileId.getHashValue() << "] " << FS->getLocation().printToString(*SrcMgr) << '\n';
      Rewriter *Rew = getRewPtr(FileId, SrcMgr);
      if (visitFunctionDecl(FS, *Rew) && MCollectFiles) {
        const FileEntry* Entry = SrcMgr->getFileEntryForID(FileId);
        const auto FileName = Entry->tryGetRealPathName();
        MFileNames.insert(FileName.str());
      }
    }
    llvm::outs() << "Done\n";
  }

  std::vector<std::string> fetchCollectedFiles() const {
    return std::vector(MFileNames.cbegin(), MFileNames.cend());
  }
};

class Callbacks : public SourceFileCallbacks {

  RewDb &MRewriterDb;

public:
  Callbacks(RewDb &RewriterDb) : MRewriterDb(RewriterDb) {}

  void handleEndSource() override {
    for (auto &&Rew : MRewriterDb) {
      llvm::outs() << "Ending file: [" << Rew.first.getHashValue() << "]\n";
      if (Rew.second.overwriteChangedFiles()) {
        llvm::outs() << "Failed to flush " << Rew.first.getHashValue() << '\n';
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
  FunctionDeclRewriter Rewriter(Rewriters, ProduceFileList.getValue());
  MatchFinder Finder;
  Finder.addMatcher(FunctionMatcher, &Rewriter);

  auto Result =  Tool.run(newFrontendActionFactory(&Finder, &Db).get());
  
  if (ProduceFileList.getValue()) {
    auto Files = Rewriter.fetchCollectedFiles();
    const auto *ListPath = "modified-files.txt";
    std::ofstream Out(ListPath);
    for (auto&& Fname : Files) {
      Out << Fname << '\n';
    }
    llvm::outs() << "Written file list into " << ListPath << '\n';
  }

  return Result;
}