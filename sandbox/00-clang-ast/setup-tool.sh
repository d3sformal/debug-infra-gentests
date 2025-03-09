#!/bin/sh

# expects llvm-project repository

mkdir llvm-project/clang-tools-extra/ast-injection

cp -r ./cpy-to-llvm-project/clang-tools-extra/ast-injection/* llvm-project/clang-tools-extra/ast-injection/

echo "add_subdirectory(ast-injection)" >> llvm-project/clang-tools-extra/CMakeLists.txt

echo "Run make in the build directory now"
