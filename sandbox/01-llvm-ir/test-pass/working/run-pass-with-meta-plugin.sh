#!/bin/sh
set -x

CLANG_COMPILER=$1; shift
TARGET=$1; shift

PLUGIN_OPTS="$@"

cp ../"$TARGET" ./
mkdir -p ./build
$CLANG_COMPILER -Xclang -load -Xclang ../../llvm-pass/libfn-pass.so -Xclang -fpass-plugin=../../llvm-pass/libfn-pass.so -fplugin=../../../build/lib/AstMetaAdd.so "$TARGET" -S -emit-llvm -o ./build/bitcode.ll $PLUGIN_OPTS

