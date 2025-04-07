#!/bin/sh
set -x

COMPILER=$1; shift
SOURCE_FILE=$1; shift

if [ -z "$COMPILER" ] || [ -z "$SOURCE_FILE" ]; then
    echo "Usage: $0 <compiler> <file to instrument>" >&2
    exit 1
fi
echo "BUILD"
./rebuild-pass.sh
./rebuild-meta-plugin.sh
echo "PASS"
./run-pass-with-meta-plugin.sh "$COMPILER" "$SOURCE_FILE"
echo "IR TO BIN"
./ir-to-bin.sh "$COMPILER"
. ./export_lib_path.sh
./build/a.out
