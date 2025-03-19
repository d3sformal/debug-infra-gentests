#include "clang/AST/Stmt.h"
#include "clang/Basic/LangOptions.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Rewrite/Core/Rewriter.h"
#include "clang/Tooling/CommonOptionsParser.h"
#include "clang/Tooling/Tooling.h"

#include <cassert>

inline clang::StringRef rangeToString(const clang::SourceRange &Range,
                                      clang::SourceManager &Sm) {
  return clang::Lexer::getSourceText(
      clang::CharSourceRange::getTokenRange(Range), Sm, clang::LangOptions(),
      0);
}
