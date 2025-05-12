#!/bin/sh
set -e

cp ../../01-llvm-ir/llvm-pass/libfn-pass.so ./
cp ../../build/lib/AstMetaAdd.so ./
cd ../ipc-hooklib
cmake ./ -DCFG_MANUAL=OFF
make
cd ../example-complex

cmake -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  ./

# the mllvm args must precede plugin loading

cmake   -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  -DCMAKE_CXX_FLAGS="-mllvm -Arg -mllvm -llcap-fn-targets-file=./selected-fns.bin -Xclang -load -Xclang ./libfn-pass.so -Xclang -fpass-plugin=./libfn-pass.so -fplugin=./AstMetaAdd.so" \
  ./

make clean
make

