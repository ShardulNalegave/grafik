cmake_minimum_required(VERSION 3.10)
project(grafik)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

add_subdirectory(vendor/glfw)
add_subdirectory(vendor/spdlog)

add_library(glad vendor/glad/src/glad.c)
target_include_directories(glad PUBLIC vendor/glad/include)

include_directories(include)
include_directories(vendor/glfw/include)
include_directories(vendor/spdlog/include)

file(GLOB_RECURSE SOURCES src/*.cpp)
file(GLOB_RECURSE HEADERS src/*.h)

add_executable(grafik ${SOURCES})

target_link_libraries(grafik glfw)
target_link_libraries(grafik glad)
target_link_libraries(grafik spdlog::spdlog)

if (APPLE)
  target_link_libraries(grafik "-framework OpenGL")
elseif(WIN32)
  target_link_libraries(grafik opengl32)
elseif(UNIX)
  target_link_libraries(grafik GL)
endif()
