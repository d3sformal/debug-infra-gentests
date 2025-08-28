cmake_minimum_required(VERSION 3.30)
project(testProgram)
set(CMAKE_CXX_STANDARD 23)

add_library(funTraceLib STATIC lib/include/funTrace.hpp lib/src/funTrace.cpp)

add_executable(testProgram main.cpp
        lib/include/funTrace.hpp
)
target_link_libraries(testProgram funTraceLib)
