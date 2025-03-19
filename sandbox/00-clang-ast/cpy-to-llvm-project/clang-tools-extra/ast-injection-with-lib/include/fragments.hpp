#include "clang/AST/Decl.h"
#include "clang/AST/ParentMap.h"
#include "clang/AST/Stmt.h"
#include "clang/Rewrite/Core/Rewriter.h"

#include <cassert>
#include <string>

#include "util.hpp"

namespace Fragments {
static constexpr const char *SScopeDumperVarName = "__function_tracking_scope";
inline std::string scopeStartTrackFragment(const std::string &FnName,
                                           uint64_t Id) {
  return std::string("\nauto ") + SScopeDumperVarName +
         " = funTraceLib::ScopeDumper(\"" + FnName + "\", " +
         std::to_string(Id) + ");\n";
}

inline std::string scopeEndTrackingFragment(const std::string &FnName,
                                            uint64_t Id) {
  return "";
}

inline std::string returnTrackFragment() {
  return std::string(SScopeDumperVarName) + ".registerReturn();\n";
}

inline std::string returnSaveTraceFragment(clang::ReturnStmt *Ret,
                                           clang::Rewriter &Rewriter,
                                           uint64_t &Counter) {
  const char *RvName = "__function_tracking_retval_";
  std::string RvNameUnique = RvName + (std::to_string(Counter++));
  std::string RetValSave =
      Ret->children().empty()
          ? std::string("")
          : std::string("auto&& ") + RvNameUnique + "(" +
                rangeToString((Ret->child_begin())->getSourceRange(),
                              Rewriter.getSourceMgr())
                    .str() +
                ");\n";

  return RetValSave + "\n" + Fragments::returnTrackFragment() + "\treturn " +
         RvNameUnique + ";\n";
}

// TODO
inline bool injectParamFragments(const clang::FunctionDecl *Func,
                                 clang::Rewriter &Rewriter) {
  bool Result = false;
  llvm::outs() << "\tParameters\n";
  for (unsigned int I = 0; I < Func->getNumParams(); I++) {
    std::string VarString = Func->parameters()[I]->getQualifiedNameAsString();
    llvm::outs() << "\t\t" << VarString << "\n";
    if (Func->getBody() == nullptr) {
      continue;
    }

    Result = !Rewriter.InsertTextAfterToken(
        Func->getBody()->getBeginLoc(), "\n::__framework::Reporter::report(" +
                                            VarString + ", \"" + VarString +
                                            "\");\n");
  }
  return Result;
}

inline std::string libraryInitFragment(const std::string &LogTarget) {
  return "auto __funtraceLibLogger = funTraceLib::TraceLogger(\"" + LogTarget +
         "\");\n";
}
}; // namespace Fragments