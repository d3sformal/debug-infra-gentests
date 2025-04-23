cmake_minimum_required(VERSION 3.30)
project(ipc-fin)

add_compile_options(-Wall -Wextra -pedantic)

find_package(PkgConfig REQUIRED)
pkg_check_modules(CZMQ REQUIRED libczmq)

include_directories(${CMAKE_SOURCE_DIR})
include_directories(${CZMQ_INCLUDE_DIRS})

add_library(hooklib SHARED IMPORTED)
set_target_properties(hooklib PROPERTIES
    IMPORTED_LOCATION ../ipc-hooklib/libmy-hook.so
)

target_link_libraries(hooklib INTERFACE ${CZMQ_LIBRARIES})


add_executable(ipc-fin ./main.c)
target_link_libraries(ipc-fin PRIVATE hooklib)
