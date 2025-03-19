#!/bin/bash

cp ../test-program.cpp ./
../../build/bin/ast-injection -F ./test-program.cpp --
clang++ ./test-program.cpp

