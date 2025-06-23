#!/bin/sh
set -e

cp ../../01-llvm-ir/llvm-pass/libfn-pass.so ./
cp ../../build/lib/AstMetaAdd.so ./
cd ../ipc-hooklib
cmake ./ -DCFG_MANUAL=OFF
make
cd ../example-arg-replacement

cmake -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  ./

# the mllvm args must precede plugin loading

cmake   -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  -D CMAKE_CXX_FLAGS="-mllvm -Call -Xclang -load -Xclang ./libfn-pass.so -Xclang -fpass-plugin=./libfn-pass.so -fplugin=./AstMetaAdd.so" \
  ./

rm ./module-maps/* || true
mkdir -p ./module-maps
make clean
make

