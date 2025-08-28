#!/bin/bash

set -e
PathToIR="$1/CMakeFiles/testbin.dir/main.cpp.o"
echo "$PathToIR"

grep -e "test_target.*LLVM-THIS" $PathToIR
