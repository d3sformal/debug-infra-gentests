#!/bin/sh

make clean

rm ./*.so
rm ./example
rm ./CMakeCache*
rm -rf ./CMakeFiles/

cmake -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  -D CMAKE_CXX_FLAGS="" \
  ./

make

mkdir -p ./module-maps
