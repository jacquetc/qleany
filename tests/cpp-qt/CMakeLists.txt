cmake_minimum_required(VERSION 3.21)
project(qleany-cpp-qt-functional-tests LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

# Make CMAKE_MODULE_PATH available for the tested project's VersionFromGit etc.
list(APPEND CMAKE_MODULE_PATH
    ${CMAKE_CURRENT_SOURCE_DIR}/tested_project/cmake/
    ${CMAKE_CURRENT_SOURCE_DIR}/tested_project/cmake/3rdparty/
)

# Build the tested project as a subdirectory so we can link against its targets
set(FUL_BUILD_TESTS OFF CACHE BOOL "" FORCE)
add_subdirectory(tested_project)

# Build our functional tests
enable_testing()
add_subdirectory(functional)