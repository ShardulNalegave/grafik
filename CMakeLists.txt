cmake_minimum_required(VERSION 3.10)
project(grafik)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

add_subdirectory(example)
add_subdirectory(vendor/glfw)
add_subdirectory(vendor/spdlog)

add_library(glad vendor/glad/src/glad.c)
target_include_directories(glad PUBLIC vendor/glad/include)

file(GLOB_RECURSE SOURCES src/*.cpp)
file(GLOB_RECURSE HEADERS src/*.h)

add_library(grafik STATIC ${SOURCES} ${HEADERS})

target_include_directories(grafik PUBLIC include)
target_include_directories(grafik PRIVATE vendor/glfw/include)
target_include_directories(grafik PRIVATE vendor/spdlog/include)

target_link_libraries(grafik PRIVATE glfw)
target_link_libraries(grafik PRIVATE glad)
target_link_libraries(grafik PRIVATE spdlog::spdlog)
