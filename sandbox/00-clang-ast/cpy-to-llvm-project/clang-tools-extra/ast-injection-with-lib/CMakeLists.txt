set(LLVM_LINK_COMPONENTS support)

add_clang_executable(ast-injection-wlib
  AstInjection.cpp
  )
target_link_libraries(ast-injection-wlib
  PRIVATE
  clangAST
  clangASTMatchers
  clangBasic
  clangFrontend
  clangSerialization
  clangTooling
  )