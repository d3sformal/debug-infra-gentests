#!/bin/sh
set -e

# expects the llvm-project submodule fully built according tho the root README

# a dumb check
LLVM_PATH=../../llvm-project
TARGET_TOOL_FOLDER=$LLVM_PATH/clang/examples/ast-meta-add
if [ -d "$TARGET_TOOL_FOLDER" ]; then
  echo "Directory already exists. This script should be run only once..."
  exit 1
fi

mkdir "$TARGET_TOOL_FOLDER"
cp -r ./ast-meta-add/* "$TARGET_TOOL_FOLDER"

# this is why you should not run this more than once - to undo this, simply undo this append
echo "add_subdirectory(ast-meta-add)" >> "$LLVM_PATH/clang/examples/CMakeLists.txt"

echo "You may run ninja in ../../build now"
echo "If you run into a Permission Denied to a file AstMetaAdd.exports when building the plugin, run ninja with root permissions (sorry, I have no idea why this error happens)"
