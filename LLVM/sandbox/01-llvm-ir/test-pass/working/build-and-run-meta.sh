#!/bin/sh
set -xe

Compiler=$1; shift
SrcFile=$1; shift

if [ -z "$Compiler" ] || [ -z "$SrcFile" ]; then
    echo "Usage: $0 <compiler> <file to instrument>" >&2
    exit 1
fi

echo "CLEANUP"
rm -f ./module-maps/*

echo "BUILD"
./rebuild-pass.sh
./rebuild-meta-plugin.sh
./build-libs.sh
echo "PASS"
./run-pass-with-meta-plugin.sh "$Compiler" "$SrcFile" $@
echo "IR TO BIN"
./ir-to-bin.sh "$Compiler"
. ./export_lib_path.sh
./build/a.out
