cmake_minimum_required(VERSION 3.30)
project(Example)
option(ARG_CAPTURE_INSTR "Instrument for argument capture")

add_compile_options(-Wall -Wextra -pedantic)

if(ARG_CAPTURE_INSTR)
add_compile_options(
  "SHELL:-mllvm -Arg" 
  "SHELL:-mllvm -llcap-fn-targets-file=./selected-fns.bin")
endif()

find_package(PkgConfig REQUIRED)
pkg_check_modules(CZMQ REQUIRED libczmq)


add_library(hooklib SHARED IMPORTED)
set_target_properties(hooklib PROPERTIES
IMPORTED_LOCATION ../ipc-hooklib/libmy-hook.so
)

include_directories(${CMAKE_SOURCE_DIR})


add_executable(example main.cpp lib.cpp lib2.cpp)

target_link_libraries(example PRIVATE hooklib)