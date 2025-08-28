#!/bin/bash

cp ../test-program.cpp ./
../../build/bin/ast-injection-wlib -M ./modified-files.txt -I=./fn-ids.csv -T ./test-program.cpp --
../../inject-w-library/lib/add_includes.sh ./modified-files.txt ../../inject-w-library/lib/include/funTrace.hpp
clang++ ./test-program.cpp ../../inject-w-library/lib/src/funTrace.cpp

