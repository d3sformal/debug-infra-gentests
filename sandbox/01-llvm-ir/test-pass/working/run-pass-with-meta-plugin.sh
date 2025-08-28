#!/bin/sh
set -xe

ClangCompiler=$1; shift
Target=$1; shift

PluginOpts="$@"

cp ../"$Target" ./
mkdir -p ./build
$ClangCompiler -Xclang -load -Xclang ../../llvm-pass/libfn-pass.so -Xclang -fpass-plugin=../../llvm-pass/libfn-pass.so -fplugin=/usr/local/lib/AstMetaAdd.so "$Target" -S -emit-llvm -o ./build/bitcode.ll $PluginOpts

