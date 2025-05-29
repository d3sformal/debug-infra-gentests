set(CMAKE_C_STANDARD 23)
option(CFG_MANUAL "Library compiled without auto initialization and deinitialization")

cmake_minimum_required(VERSION 3.30)
project(my-hook)

add_compile_options(-Wall -Wextra -Wconversion -Wpedantic)

find_package(PkgConfig REQUIRED)
pkg_check_modules(CZMQ REQUIRED libczmq)

include_directories(${CMAKE_SOURCE_DIR})
include_directories(${CZMQ_INCLUDE_DIRS})

add_library(my-hook SHARED ./hook.cpp ./shm.c ./shm_write_channel.c ./shm_util.c)
target_link_libraries(my-hook ${CZMQ_LIBRARIES})

if(CFG_MANUAL)
add_library(my-hook-manual SHARED ./hook.cpp ./shm.c ./shm_write_channel.c ./shm_util.c)
target_link_libraries(my-hook-manual ${CZMQ_LIBRARIES})
set_target_properties(my-hook-manual PROPERTIES COMPILE_DEFINITIONS "MANUAL_INIT_DEINIT=1")
endif()
