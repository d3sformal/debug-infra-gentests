#include "clang/AST/Decl.h"
#include "clang/AST/ParentMap.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/Stmt.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Basic/LangOptions.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Rewrite/Core/Rewriter.h"
#include "clang/Tooling/CommonOptionsParser.h"
#include "clang/Tooling/Tooling.h"

#include "./include/fragments.hpp"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/raw_ostream.h"
#include <algorithm>
#include <cassert>
#include <map>
#include <set>
#include <string>
#include <utility>

using namespace clang::tooling;
using namespace llvm;
using namespace clang;

// Apply a custom category to all command-line options so that they are the
// only ones displayed.
static llvm::cl::OptionCategory MyToolCategory("Tool options");

static cl::opt<bool>
    ProduceFileList("F", cl::desc("Produce a list of modified files"),
                    cl::cat(MyToolCategory));

static cl::opt<std::string>
    ProduceFnIdMap("I", cl::desc("Output function id mapping to a file"),
                      cl::cat(MyToolCategory));

static cl::opt<bool> Verbose("v", cl::desc("More detailed logging"),
                             cl::cat(MyToolCategory));

// CommonOptionsParser declares HelpMessage with a description of the common
// command-line options related to the compilation database and input files.
// It's nice to have this help message in all tools.
static cl::extrahelp CommonHelp(CommonOptionsParser::HelpMessage);

// A help message for this specific tool can be added afterwards.
static cl::extrahelp MoreHelp("\nMore help text...\n");

static CFunctionRegistry<uint64_t, std::string> FunctionIdProvider;

static bool injectScopeTrackingFragments(const FunctionDecl *Func,
                                         Rewriter &Rewriter) {
  auto FnName = Func->getQualifiedNameAsString();
  auto FnFilePath = Func->getLocation().printToString(Rewriter.getSourceMgr());
  return !Rewriter.InsertTextAfterToken(
      Func->getBody()->getBeginLoc(),
      Fragments::scopeStartTrackFragment(FnName, FunctionIdProvider.getFunctionId(FnFilePath + ' ' + FnName)));
}

class ReturnCollector : public clang::RecursiveASTVisitor<ReturnCollector> {
public:
  // shouldVisitLambdaBody
  // shouldVisitTemplateInstantiations
  static constexpr auto RetStmtBuffSize = 5;
  using Returns = llvm::SmallVector<clang::ReturnStmt *, RetStmtBuffSize>;

  static Returns collect(const clang::FunctionDecl *Func) {
    ReturnCollector ActualCollector;
    ActualCollector.TraverseDecl(const_cast<clang::FunctionDecl *>(Func));
    return ActualCollector.Visited;
  }

  bool VisitReturnStmt(clang::ReturnStmt *R) {
    Visited.push_back(R);
    return true;
  }

private:
  ReturnCollector() = default;
  Returns Visited;
};

static bool injectReturnTrackingFragments(const FunctionDecl *Func,
                                          Rewriter &Rewriter) {
  uint64_t RetStmtCounter = 0;
  auto Returns = ReturnCollector::collect(Func);
  if (Returns.empty()) {
    Rewriter.InsertTextBefore(Func->getBody()->getEndLoc(),
                              Fragments::returnTrackFragment());
    return true;
  }

  auto InsertReturnTrackFragmentBeforeRetStmt = [&](ReturnStmt *Ret) {
    Rewriter.InsertTextAfterToken(Ret->getBeginLoc().getLocWithOffset(-1),
                                  Fragments::returnTrackFragment());
  };
  if (std::all_of(Returns.begin(), Returns.end(),
                  [](ReturnStmt *R) { return R->children().empty(); })) {
    // all return statements are plain (without children -> fn returns void)
    for (auto &&Ret : Returns) {
      InsertReturnTrackFragmentBeforeRetStmt(Ret);
    }
  } else {
    auto Parents = ParentMap(Func->getBody());
    for (auto &&Ret : Returns) {
      auto *Parent = Parents.getParent(Ret);
      auto Target = std::find_if(
          Parent->child_begin(), Parent->child_end(), [Ret](auto *C) {
            if (auto *RetChild = dyn_cast<clang::ReturnStmt>(C)) {
              return RetChild == Ret;
            }
            return false;
          });

      auto InnerFragment =
          Fragments::returnSaveTraceFragment(Ret, Rewriter, RetStmtCounter);

      if (!isa<CompoundStmt>(*Parent)) {
        std::string Replacement = "{\n" + InnerFragment + "}";

        Rewriter.ReplaceText(Target->getSourceRange(), Replacement);
      } else {
        Rewriter.ReplaceText(Target->getSourceRange(), InnerFragment);
      }
    }
  }
  return true;
}

static bool visitFunctionDecl(const FunctionDecl *Func, Rewriter &Rewriter) {
  bool Result = false;
  if (Verbose.getValue())
    llvm::errs() << "Visiting " << Func->getQualifiedNameAsString() << '\n';
  if (!Func->hasBody()) {
    llvm::errs() << Func->getQualifiedNameAsString() << ": no body\n";
    return Result;
  }

  Result |= injectScopeTrackingFragments(Func, Rewriter);
  Result |= injectReturnTrackingFragments(Func, Rewriter);
  return Result;
}

static bool visitMainDecl(const FunctionDecl *Func, Rewriter &Rewriter) {
  // TODO make this error if called twice
  if (!Func->hasBody()) {
    llvm::errs() << Func->getQualifiedNameAsString() << ": no body\n";
    // TODO terminate
    return false;
  }

  // TODO path
  Rewriter.InsertText(Func->getBody()->getBeginLoc().getLocWithOffset(1),
                      Fragments::libraryInitFragment("./log.txt"));
  return true;
}

using namespace clang;
using namespace clang::ast_matchers;

static DeclarationMatcher FunctionMatcher =
    anyOf(functionDecl(allOf(isExpansionInMainFile(),
                             anyOf(hasAnyParameter(hasType(asString("float"))),
                                   hasAnyParameter(hasType(asString("int"))))))
              .bind("functionDecl"),
          functionDecl(hasName("::main")).bind("mainDecl"));

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
  FunctionDeclRewriter(RewDb &RewDb, bool CollectFiles = false)
      : MRewDb(RewDb), MCollectFiles(CollectFiles) {}

  virtual void run(const MatchFinder::MatchResult &Result) override {
    // TODO deduplicate
    assert(Result.SourceManager != nullptr);
    if (const FunctionDecl *FS =
            Result.Nodes.getNodeAs<clang::FunctionDecl>("functionDecl")) {
      // file id
      auto *SrcMgr = Result.SourceManager;
      auto FileId = SrcMgr->getFileID(FS->getLocation());
      if (Verbose.getValue())
        llvm::errs() << "Trying file: [" << FileId.getHashValue() << "] "
                     << FS->getLocation().printToString(*SrcMgr) << '\n';
      Rewriter *Rew = getRewPtr(FileId, SrcMgr);
      if (visitFunctionDecl(FS, *Rew) && MCollectFiles) {
        const FileEntry *Entry = SrcMgr->getFileEntryForID(FileId);
        const auto FileName = Entry->tryGetRealPathName();
        MFileNames.insert(FileName.str());
      }
    } else if (const FunctionDecl *FS =
                   Result.Nodes.getNodeAs<clang::FunctionDecl>("mainDecl")) {
      auto *SrcMgr = Result.SourceManager;
      auto FileId = SrcMgr->getFileID(FS->getLocation());
      if (Verbose.getValue())
        llvm::errs() << "File where we found main: [" << FileId.getHashValue()
                     << "] " << FS->getLocation().printToString(*SrcMgr)
                     << '\n';
      Rewriter *Rew = getRewPtr(FileId, SrcMgr);
      if (visitMainDecl(FS, *Rew) && MCollectFiles) {
        const FileEntry *Entry = SrcMgr->getFileEntryForID(FileId);
        const auto FileName = Entry->tryGetRealPathName();
        MFileNames.insert(FileName.str());
      }
    }
    if (Verbose.getValue())
      llvm::errs() << "Done\n";
  }

  std::vector<std::string> fetchModifiedFiles() const {
    return std::vector(MFileNames.cbegin(), MFileNames.cend());
  }
};

class Callbacks : public SourceFileCallbacks {

  RewDb &MRewriterDb;

public:
  Callbacks(RewDb &RewriterDb) : MRewriterDb(RewriterDb) {}

  void handleEndSource() override {
    for (auto &&Rew : MRewriterDb) {
      if (Verbose.getValue())
        llvm::errs() << "Ending file: [" << Rew.first.getHashValue() << "]\n";
      if (Rew.second.overwriteChangedFiles()) {
        llvm::errs() << "Failed to flush " << Rew.first.getHashValue() << '\n';
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

  auto Result = Tool.run(newFrontendActionFactory(&Finder, &Db).get());

  if (ProduceFileList.getValue()) {
    auto* ListPath = "modified-files.txt";
    if (dumpLines(Rewriter.fetchModifiedFiles(), ListPath) ) {
      llvm::errs() << "Written modified file list into " << ListPath << '\n';
    } else {
      llvm::errs() << "Failed to write modified file list into " << ListPath << '\n';
    }
  }

  if (ProduceFnIdMap.hasArgStr()) {
    auto FileName = ProduceFnIdMap.getValue();
    const auto Pairs = FunctionIdProvider.fetchFunctionIdMapping(); 
    std::vector<std::string> Lines;
    Lines.reserve(Pairs.size());
    std::transform(Pairs.begin(), Pairs.end(), std::back_inserter(Lines), [](const std::pair<std::string, int64_t> p) { return p.first + ',' + std::to_string(p.second); });

    if (dumpLines(Lines, FileName) ) {
      llvm::errs() << "Written function ID csv into" << FileName << '\n';
    } else {
      llvm::errs() << "Failed to write function ID csv into " << FileName << '\n';
    }
  }

  return Result;
}