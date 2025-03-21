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
using namespace ast_matchers;

template <class T> using vec = std::vector<T>;
using str = std::string;

// Apply a custom category to all command-line options so that they are the
// only ones displayed.
static cl::OptionCategory MyToolCategory("Tool options");

static cl::opt<str>
    ProduceFileList("M",
                    cl::desc("Output a list of modified files into a file"),
                    cl::cat(MyToolCategory));

static cl::opt<str>
    ProduceFnIdMap("I", cl::desc("Output function id mapping into a file"),
                   cl::cat(MyToolCategory));

static cl::opt<bool> Verbose("v", cl::desc("More detailed logging"),
                             cl::cat(MyToolCategory));

// CommonOptionsParser declares HelpMessage with a description of the common
// command-line options related to the compilation database and input files.
// It's nice to have this help message in all tools.
static cl::extrahelp CommonHelp(CommonOptionsParser::HelpMessage);

// A help message for this specific tool can be added afterwards.
static cl::extrahelp MoreHelp("\nMore help text...\n");

static CFunctionRegistry<uint64_t, str> FunctionIdProvider;

static bool injectScopeTrackingFragments(const FunctionDecl *Func,
                                         Rewriter &Rewriter) {
  auto FnName = Func->getQualifiedNameAsString();
  auto FnFilePath = Func->getLocation().printToString(Rewriter.getSourceMgr());
  return !Rewriter.InsertTextAfterToken(
      Func->getBody()->getBeginLoc(),
      Fragments::scopeStartTrackFragment(
          FnName, FunctionIdProvider.getFunctionId(FnFilePath + ' ' + FnName)));
}

class ReturnCollector : public RecursiveASTVisitor<ReturnCollector> {
public:
  // shouldVisitLambdaBody
  // shouldVisitTemplateInstantiations
  static constexpr auto RetStmtBuffSize = 5;
  using Returns = SmallVector<ReturnStmt *, RetStmtBuffSize>;

  static Returns collect(const FunctionDecl *Func) {
    ReturnCollector ActualCollector;
    ActualCollector.TraverseDecl(const_cast<FunctionDecl *>(Func));
    return ActualCollector.Visited;
  }

  bool VisitReturnStmt(ReturnStmt *R) {
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
            if (auto *RetChild = dyn_cast<ReturnStmt>(C)) {
              return RetChild == Ret;
            }
            return false;
          });

      auto InnerFragment =
          Fragments::returnSaveTraceFragment(Ret, Rewriter, RetStmtCounter);

      if (!isa<CompoundStmt>(*Parent)) {
        str Replacement = "{\n" + InnerFragment + "}";

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
    errs() << "Visiting " << Func->getQualifiedNameAsString() << '\n';
  if (!Func->hasBody()) {
    errs() << Func->getQualifiedNameAsString() << ": no body\n";
    return Result;
  }

  Result |= injectScopeTrackingFragments(Func, Rewriter);
  Result |= injectReturnTrackingFragments(Func, Rewriter);
  return Result;
}

static bool visitMainDecl(const FunctionDecl *Func, Rewriter &Rewriter) {
  // TODO make this error if called twice
  if (!Func->hasBody()) {
    errs() << Func->getQualifiedNameAsString() << ": no body\n";
    // TODO terminate
    return false;
  }

  // TODO path
  Rewriter.InsertText(Func->getBody()->getBeginLoc().getLocWithOffset(1),
                      Fragments::libraryInitFragment("./log.txt"));
  return true;
}

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

  std::set<str> MFileNames;
  bool MCollectFiles{false};

public:
  FunctionDeclRewriter(RewDb &RewDb, bool CollectFiles = false)
      : MRewDb(RewDb), MCollectFiles(CollectFiles) {}

  virtual void run(const MatchFinder::MatchResult &Result) override {
    // TODO deduplicate
    assert(Result.SourceManager != nullptr);
    if (const FunctionDecl *FS =
            Result.Nodes.getNodeAs<FunctionDecl>("functionDecl")) {
      // file id
      auto *SrcMgr = Result.SourceManager;
      auto FileId = SrcMgr->getFileID(FS->getLocation());
      if (Verbose.getValue())
        errs() << "Trying file: [" << FileId.getHashValue() << "] "
               << FS->getLocation().printToString(*SrcMgr) << '\n';
      Rewriter *Rew = getRewPtr(FileId, SrcMgr);
      if (visitFunctionDecl(FS, *Rew) && MCollectFiles) {
        const FileEntry *Entry = SrcMgr->getFileEntryForID(FileId);
        const auto FileName = Entry->tryGetRealPathName();
        MFileNames.insert(FileName.str());
      }
    } else if (const FunctionDecl *FS =
                   Result.Nodes.getNodeAs<FunctionDecl>("mainDecl")) {
      auto *SrcMgr = Result.SourceManager;
      auto FileId = SrcMgr->getFileID(FS->getLocation());
      if (Verbose.getValue())
        errs() << "File where we found main: [" << FileId.getHashValue() << "] "
               << FS->getLocation().printToString(*SrcMgr) << '\n';
      Rewriter *Rew = getRewPtr(FileId, SrcMgr);
      if (visitMainDecl(FS, *Rew) && MCollectFiles) {
        const FileEntry *Entry = SrcMgr->getFileEntryForID(FileId);
        const auto FileName = Entry->tryGetRealPathName();
        MFileNames.insert(FileName.str());
      }
    }
    if (Verbose.getValue())
      errs() << "Done\n";
  }

  vec<str> fetchModifiedFiles() const {
    return vec<str>(MFileNames.cbegin(), MFileNames.cend());
  }
};

class Callbacks : public SourceFileCallbacks {

  RewDb &MRewriterDb;

public:
  Callbacks(RewDb &RewriterDb) : MRewriterDb(RewriterDb) {}

  void handleEndSource() override {
    for (auto &&Rew : MRewriterDb) {
      if (Verbose.getValue())
        errs() << "Ending file: [" << Rew.first.getHashValue() << "]\n";
      if (Rew.second.overwriteChangedFiles()) {
        errs() << "Failed to flush " << Rew.first.getHashValue() << '\n';
      }
    }
  }
};

void dumpVec(const vec<str> &Vec, const str &FileName) {
  if (dumpLines(Vec, FileName)) {
    errs() << "Written modified file list into " << FileName << '\n';
  } else {
    errs() << "Failed to write modified file list into " << FileName << '\n';
  }
}

int main(int argc, const char **argv) {
  auto ExpectedParser = CommonOptionsParser::create(argc, argv, MyToolCategory);
  if (!ExpectedParser) {
    // Fail gracefully for unsupported options.
    errs() << ExpectedParser.takeError();
    return 1;
  }
  CommonOptionsParser &OptionsParser = ExpectedParser.get();
  ClangTool Tool(OptionsParser.getCompilations(),
                 OptionsParser.getSourcePathList());

  RewDb Rewriters = {};
  Callbacks Db(Rewriters);
  FunctionDeclRewriter Rewriter(Rewriters, ProduceFileList.hasArgStr());
  MatchFinder Finder;
  Finder.addMatcher(FunctionMatcher, &Rewriter);

  auto Result = Tool.run(newFrontendActionFactory(&Finder, &Db).get());

  if (ProduceFileList.hasArgStr()) {
    auto FileName = ProduceFileList.getValue();
    if (dumpLines(Rewriter.fetchModifiedFiles(), FileName)) {
      errs() << "Written modified file list into " << FileName << '\n';
    } else {
      errs() << "Failed to write modified file list into " << FileName << '\n';
    }
  }

  if (ProduceFnIdMap.hasArgStr()) {
    auto FileName = ProduceFnIdMap.getValue();
    const auto Pairs = FunctionIdProvider.fetchFunctionIdMapping();
    vec<str> Lines;
    Lines.reserve(Pairs.size());
    std::transform(Pairs.begin(), Pairs.end(), std::back_inserter(Lines),
                   [](const std::pair<str, uint64_t> &P) {
                     return P.first + ',' + std::to_string(P.second);
                   });

    if (dumpLines(Lines, FileName)) {
      errs() << "Written function ID csv into" << FileName << '\n';
    } else {
      errs() << "Failed to write function ID csv into " << FileName << '\n';
    }
  }

  return Result;
}