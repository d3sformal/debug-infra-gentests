#!/bin/sh
set -x

ClangCompiler=$1; shift
Target=$1; shift
PluginOpts="$@"

cp ../"$Target" ./
mkdir -p ./build
$ClangCompiler -Xclang -load -Xclang ../../llvm-pass/libfn-pass.so -Xclang -fpass-plugin=../../llvm-pass/libfn-pass.so "$Target" -S -emit-llvm -o ./build/bitcode.ll $PluginOpts -mllvm -llcap-filter-by-mangled

