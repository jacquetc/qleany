cmake_minimum_required(VERSION 3.21)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake)

# get version from git
include(VersionFromGit)
version_from_git(LOG ON)
add_compile_definitions(QLEANY_VERSION_SEMANTIC="${SEMVER}")
set(QLEANY_VERSION_SEMANTIC ${SEMVER})
add_compile_definitions(QLEANY_VERSION="${VERSION}")
set(QLEANY_VERSION ${VERSION})
add_compile_definitions(QLEANY_VERSION_MAJOR=${VERSION_MAJOR})
set(QLEANY_VERSION_MAJOR ${VERSION_MAJOR})
add_compile_definitions(QLEANY_VERSION_MINOR=${VERSION_MINOR})
set(QLEANY_VERSION_MINOR ${VERSION_MINOR})
add_compile_definitions(QLEANY_VERSION_PATCH=${VERSION_PATCH})
set(QLEANY_VERSION_PATCH ${VERSION_PATCH})

project(
  Qleany
  VERSION ${QLEANY_VERSION}
  LANGUAGES CXX
  DESCRIPTION "Qleany is a generator for Clean Architecture with Qt6")
string(TOLOWER ${PROJECT_NAME} PROJECT_NAME_LOWER)

# Compiler settings
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(QT_MIN_VERSION "6.4.0")
set(QT_DEFAULT_MAJOR_VERSION 6)
set(CMAKE_AUTOMOC ON)

add_compile_definitions(
  QT_NO_KEYWORDS
  QT_NO_CAST_TO_ASCII
  QT_NO_CAST_FROM_ASCII
  QT_NO_CAST_FROM_BYTEARRAY
  QT_NO_KEYWORDS
  QT_NO_FOREACH
  QT_STRICT_ITERATORS
  QT_USE_QSTRINGBUILDER
  QT_NO_NARROWING_CONVERSIONS_IN_CONNECT
  QT_NO_URL_CAST_FROM_STRING)

if(NOT WIN32)
  # strict iterators on MSVC only work when Qt itself is also built with them,
  # which is not usually the case. Otherwise there are linking issues.
  add_compile_definitions(QT_STRICT_ITERATORS)
endif()

# options
option(QLEANY_BUILD_EXAMPLES "Build examples" ON)
option(QLEANY_BUILD_TESTS "Build tests" ON)
option(BUILD_SHARED_LIBS "Build shared libraries" OFF)

if(QLEANY_BUILD_WITH_QT_GUI)
  add_compile_definitions(QLEANY_BUILD_WITH_QT_GUI=true)
endif()

if(BUILD_SHARED_LIBS)
  set(LIB_TYPE SHARED)
else()
  set(LIB_TYPE STATIC)
endif()

# Add Qt components
find_package(
  Qt6
  COMPONENTS Core Sql Gui Widgets Concurrent
  REQUIRED)
qt_standard_project_setup()

find_package(QCoro6 REQUIRED COMPONENTS Core)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

# Subdirectories that contain CMakeLists.txt for the framework tests and example
if(QLEANY_BUILD_TESTS)
  #add_subdirectory(tests)
endif()

if(QLEANY_BUILD_EXAMPLES)
  add_subdirectory(examples)
endif()
