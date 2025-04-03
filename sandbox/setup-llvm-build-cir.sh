#!/bin/sh
set -e
# before excution - if in llvm-project
# cd llvm-project
# git remote add llvm-clangir https://github.com/llvm/clangir.git
# git fetch llvm-clangir --depth=1
# git checkout -b clangir llvm-clangir/main
# cd ../
# or without submodule:
# git clone https://github.com/llvm/clangir.git llvm-project-cir

mkdir build-cir
cd build-cir
cmake \
    -G Ninja ../llvm-project-cir/llvm \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
    -DLLVM_ENABLE_PROJECTS="clang;mlir;clang-tools-extra" \
    -DCMAKE_BUILD_TYPE=Release \
    -DLLVM_BUILD_TESTS=OFF \
    -DLLDB_INCLUDE_TESTS=OFF \
    -DCLANG_INCLUDE_TESTS=OFF \
    -DLLVM_TARGETS_TO_BUILD=host \
    -DLLVM_INSTALL_UTILS=ON \
    -DLLVM_ENABLE_DUMP=ON \
    -DCLANG_ENABLE_CIR=ON
    
cd ../
