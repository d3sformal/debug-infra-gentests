cmake_minimum_required(VERSION 3.30)
project(ipc-fin)

add_compile_options(-Wall -Wextra -pedantic)

add_library(hooklib SHARED IMPORTED)
set_target_properties(hooklib PROPERTIES
    IMPORTED_LOCATION ../ipc-hooklib/libmy-hook-manual.so
)

add_executable(ipc-fin ./main.c)
target_link_libraries(ipc-fin PRIVATE hooklib)
