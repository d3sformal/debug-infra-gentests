#!/bin/sh
set -e

# expects the llvm-project submodule fully built according tho the root README

# a dumb check
TARGET_TOOL_FOLDER=../llvm-project/clang-tools-extra/ast-injection-with-lib/
if [ -d "$TARGET_TOOL_FOLDER" ]; then
  echo "Directory already exists. This script should be run only once..."
  exit 1
fi
TARGET_TOOL_FOLDER=../llvm-project/clang-tools-extra/

cp -r ./cpy-to-llvm-project/clang-tools-extra/* "$TARGET_TOOL_FOLDER"

echo "add_subdirectory(ast-injection)"          >> ../llvm-project/clang-tools-extra/CMakeLists.txt
echo "add_subdirectory(ast-injection-with-lib)" >> ../llvm-project/clang-tools-extra/CMakeLists.txt

echo "You may run ninja in ../build now" 
echo "IMPORTANT: unless you modify llvm-project/clang-tools-extra, you should NOT run this script again"
