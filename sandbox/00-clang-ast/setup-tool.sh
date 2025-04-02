#!/bin/sh

# expects the llvm-project submodule

TARGET_TOOL_FOLDER=llvm-project/clang-tools-extra/ast-injection-with-lib/

mkdir $TARGET_TOOL_FOLDER

cp -r ./cpy-to-llvm-project/clang-tools-extra/ast-injection-with-lib/* $TARGET_TOOL_FOLDER

echo "add_subdirectory(ast-injection-with-lib)" >> llvm-project/clang-tools-extra/CMakeLists.txt

echo "Run make in the build directory now"
