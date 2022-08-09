cmake_minimum_required(VERSION 3.22)
project(mcl_bench)

message("UALAL: ${CMAKE_CURRENT_LIST_DIR}")

set(CMAKE_CXX_STANDARD 14)

add_executable(mcl_bench main.cpp)

# MCL
add_library(mcl::mcl STATIC IMPORTED)
set_target_properties(mcl::mcl PROPERTIES
        IMPORTED_LOCATION "${CMAKE_CURRENT_LIST_DIR}/mcl/lib/libmcl.a"
        INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_LIST_DIR}/mcl/include"
        )
target_link_libraries(mcl_bench mcl::mcl)


# Benchmark
add_library(benchmark::benchmark STATIC IMPORTED)
set_target_properties(benchmark::benchmark PROPERTIES
        IMPORTED_LOCATION "${CMAKE_CURRENT_LIST_DIR}/benchmark/build/src/libbenchmark.a"
        INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_LIST_DIR}/benchmark/include"
        )
target_link_libraries(mcl_bench benchmark::benchmark)
