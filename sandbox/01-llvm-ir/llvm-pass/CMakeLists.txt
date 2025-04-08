cmake_minimum_required(VERSION 3.30)
project(llvm-pass)
set(CMAKE_CXX_STANDARD 23)

add_compile_options(-Wall -Wextra -pedantic)

set(NO_RTTI "-fno-rtti")
add_definitions(${NO_RTTI})

add_library(fn-pass SHARED ./pass.cpp)

