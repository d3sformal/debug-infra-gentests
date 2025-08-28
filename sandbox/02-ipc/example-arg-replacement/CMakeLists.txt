cmake_minimum_required(VERSION 3.30)
project("Argument replacement example")

add_compile_options(-Wall -Wextra -pedantic)

find_package(PkgConfig REQUIRED)

add_library(hooklib SHARED IMPORTED)
set_target_properties(hooklib PROPERTIES
IMPORTED_LOCATION ../ipc-hooklib/libmy-hook.so
)

include_directories(${CMAKE_SOURCE_DIR})


add_executable(arg-replacement main.cpp)

target_link_libraries(arg-replacement PRIVATE hooklib)