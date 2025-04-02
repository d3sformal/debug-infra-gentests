cmake_minimum_required(VERSION 3.30)
project(my-hook)

add_compile_options(-Wall -Wextra -pedantic)

add_library(my-hook SHARED ./hook.c)
