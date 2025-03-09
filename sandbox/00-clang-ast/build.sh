#!/bin/sh

cd build
make
make check
make clang-test
