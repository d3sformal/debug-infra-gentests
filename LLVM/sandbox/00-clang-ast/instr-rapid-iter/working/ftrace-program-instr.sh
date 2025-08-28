#!/bin/bash

cp ../test-program.cpp ./
../../build/bin/ast-injection -M ./modified-files.txt -I=./fn-ids.csv ./test-program.cpp --
../../inject-w-library/lib/add_includes.sh ./modified-files.txt ../../inject-w-library/lib/include/funTrace.hpp
clang++ ./test-program.cpp ../../inject-w-library/lib/src/funTrace.cpp

