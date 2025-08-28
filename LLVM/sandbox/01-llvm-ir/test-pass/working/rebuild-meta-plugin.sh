#!/bin/sh
set -x

# either run this (rebuilds every time :( )
#cd ../../custom-metadata-pass
#./cp-to-llvm-proj.sh
#cd ../../build
# or this
cd ../../../build

ninja -j4
cd ../01-llvm-ir/test-pass/working
