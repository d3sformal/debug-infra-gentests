set(CMAKE_C_STANDARD 23)
option(CFG_MANUAL "Library compiled without auto initialization and deinitialization")

cmake_minimum_required(VERSION 3.30)
project(my-hook)

add_compile_options(-Wall -Wextra -Wconversion -Wpedantic)


add_library(my-hook SHARED ./hook.cpp ./shm.c ./shm_write_channel.c ./shm_util.c)

if(CFG_MANUAL)
add_library(my-hook-manual SHARED ./hook.cpp ./shm.c ./shm_write_channel.c ./shm_util.c)
set_target_properties(my-hook-manual PROPERTIES COMPILE_DEFINITIONS "MANUAL_INIT_DEINIT=1")
endif()
