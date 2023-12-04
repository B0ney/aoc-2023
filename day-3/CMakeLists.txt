cmake_minimum_required(VERSION 3.0.0)
project(day-3 VERSION 0.1.0 LANGUAGES C CXX)

include(CTest)
enable_testing()

add_executable(part-1 part-1.cpp)
add_executable(part-2 part-2.cpp)


set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
