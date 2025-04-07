#!/bin/sh
set -x

cp ../"$2" ./
mkdir -p ./build
$1 -fpass-plugin=../../llvm-pass/libfn-pass.so -fplugin=../../../build/lib/AstMetaAdd.so "$2" -S -emit-llvm -o ./build/bitcode.ll
