cmake_minimum_required(VERSION 3.30)
project(my-hook)

add_compile_options(-Wall -Wextra -Wconversion -pedantic)

add_library(my-hook SHARED ./hook.cpp ./shm.c)

# for a failed experiment with ZMQ
# add_library(my-hook SHARED ./hook.cpp ./ipc.c)