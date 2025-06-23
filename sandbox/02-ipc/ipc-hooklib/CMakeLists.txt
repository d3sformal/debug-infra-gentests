set(CMAKE_C_STANDARD 23)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_C_COMPILER "clang")
set(CMAKE_CXX_COMPILER "clang++")
option(CFG_MANUAL "Library compiled without auto initialization and deinitialization")


cmake_minimum_required(VERSION 3.30)
project(my-hook)

add_compile_options(-Wall -Wextra -Wconversion -Wpedantic -Weverything -Wno-declaration-after-statement -Wno-pre-c23-compat -Wno-c++98-compat -Wno-pre-c11-compat -Wno-global-constructors -Wno-c++98-compat-pedantic)

add_library(my-hook SHARED ./hook.cpp ./shm.c ./shm_write_channel.c ./shm_util.c ./shm_oneshot_rx.c)
set_target_properties(my-hook PROPERTIES C_STANDARD 23)
set_target_properties(my-hook PROPERTIES CXX_STANDARD 23)

if(CFG_MANUAL)
add_library(my-hook-manual SHARED ./hook.cpp ./shm.c ./shm_write_channel.c ./shm_util.c ./shm_oneshot_rx.c)
set_target_properties(my-hook-manual PROPERTIES COMPILE_DEFINITIONS "MANUAL_INIT_DEINIT=1")
endif()
