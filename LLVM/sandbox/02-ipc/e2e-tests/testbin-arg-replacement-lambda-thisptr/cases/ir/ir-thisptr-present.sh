#!/bin/bash

set -e
PathToIR="$1/CMakeFiles/testbin.dir/main.cpp.o"
echo "$PathToIR"

# IR contains mangled names only
# $_0cl = name mangling for the first lambda's operator()
# hopefully stable enough

grep -e "\$_0cl.*LLVM-THIS" $PathToIR
