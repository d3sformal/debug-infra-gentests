set(LLVM_LINK_COMPONENTS support)

add_clang_executable(ast-injection
  AstInjection.cpp
  )
target_link_libraries(ast-injection
  PRIVATE
  clangAST
  clangASTMatchers
  clangBasic
  clangFrontend
  clangSerialization
  clangTooling
  )