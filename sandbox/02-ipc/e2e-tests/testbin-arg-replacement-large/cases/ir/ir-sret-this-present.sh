#!/bin/bash

set -e
PathToIR="$1/CMakeFiles/testbin.dir/main.cpp.o"
echo "$PathToIR"

grep -e "test_target.*ptr.*sret" $PathToIR | grep -e "LLVM-THIS"
