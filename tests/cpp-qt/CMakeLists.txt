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

# Enable testing before adding subdirectories so that both embedded and
# functional tests are registered with ctest
enable_testing()

# Build the tested project as a subdirectory so we can link against its targets.
# FUL_BUILD_TESTS is controlled via cmake -D flag from run_tests.sh;
# when ON, tested_project/tests/ is included and its add_test() calls register
# the embedded (generated) tests.
add_subdirectory(tested_project)

# Build our functional tests
add_subdirectory(functional)