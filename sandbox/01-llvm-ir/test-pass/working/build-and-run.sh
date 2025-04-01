#!/bin/bash

COMPILER=$1; shift
SOURCE_FILE=$1; shift

if [ -z "$COMPILER" ] || [ -z "$SOURCE_FILE" ]; then
    echo "Usage: $0 <compiler> <file to instrument>" >&2
    exit 1
fi

./rebuild-pass.sh
./run-pass.sh $COMPILER $SOURCE_FILE 
./ir-to-bin.sh $COMPILER 
. ./export_lib_path.sh
./build/a.out
