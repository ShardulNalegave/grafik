cmake_minimum_required(VERSION 3.10)
project(grafik-example)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

file(GLOB_RECURSE SOURCES src/*.cpp)
file(GLOB_RECURSE HEADERS src/*.h)

include_directories(../include)
include_directories(../vendor/spdlog/include)

add_executable(grafik-example ${SOURCES} ${HEADERS})
target_link_libraries(grafik-example grafik)
target_link_libraries(grafik-example spdlog::spdlog)

if (APPLE)
  target_link_libraries(grafik-example "-framework OpenGL")
elseif(WIN32)
  target_link_libraries(grafik-example opengl32)
elseif(UNIX)
  target_link_libraries(grafik-example GL)
endif()