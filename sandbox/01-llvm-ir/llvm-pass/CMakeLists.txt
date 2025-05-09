cmake_minimum_required(VERSION 3.30)
project(llvm-pass)
set(CMAKE_CXX_STANDARD 23)

add_compile_options(-Wall -Wextra -pedantic -Wconversion)

set(NO_RTTI "-fno-rtti")
add_definitions(${NO_RTTI})

add_library(fn-pass SHARED ./src/pass.cpp ./src/encoding.cpp ./src/mapping.cpp)

