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
  DESCRIPTION "Qleany is a light framework for Clean Architecture with Qt6")
string(TOLOWER ${PROJECT_NAME} PROJECT_NAME_LOWER)

# Compiler settings
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(QT_MIN_VERSION "6.4.0")
set(QT_DEFAULT_MAJOR_VERSION 6)
set(CMAKE_AUTOMOC ON)

add_compile_definitions(-DQT_NO_KEYWORDS)

# options
option(QLEANY_BUILD_EXAMPLES "Build examples" ON)
option(QLEANY_BUILD_TESTS "Build tests" ON)
option(BUILD_SHARED_LIBS "Build shared libraries" OFF)
option(QLEANY_BUILD_WITH_QT_GUI "Build with Qt GUI" ON)

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
  COMPONENTS Core Sql Concurrent
  REQUIRED)
qt_standard_project_setup()

if(QLEANY_BUILD_WITH_QT_GUI)
  find_package(
    Qt6
    COMPONENTS Gui
    REQUIRED)
endif()

if(QLEANY_BUILD_EXAMPLES)
  find_package(
    Qt6
    COMPONENTS Widgets
    REQUIRED)
endif()

find_package(QCoro6 REQUIRED COMPONENTS Core)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

# configure the library
file(GLOB_RECURSE SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB_RECURSE PUBLIC_HEADERS
     "${CMAKE_CURRENT_SOURCE_DIR}/include/qleany/*.h")

message(STATUS "PUBLIC_HEADERS: ${PUBLIC_HEADERS}")

# Create the library target
add_library(qleany ${LIB_TYPE} ${SOURCES} ${PUBLIC_HEADERS})

# Generate export header
message(STATUS "Generating export header")

include(GenerateExportHeader)
generate_export_header(
  qleany EXPORT_FILE_NAME
  ${CMAKE_CURRENT_BINARY_DIR}/include/qleany/qleany_export.h)

target_include_directories(qleany PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/include)

target_include_directories(
  qleany
  PUBLIC $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}/qleany>
         $<BUILD_INTERFACE:${CMAKE_CURRENT_BINARY_DIR}/include>
         $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>)

set_target_properties(qleany PROPERTIES CXX_STANDARD 20)

set_target_properties(qleany PROPERTIES OUTPUT_NAME qleany)
set_target_properties(
  qleany
  PROPERTIES
    PUBLIC_HEADER
    "${PUBLIC_HEADERS};${CMAKE_CURRENT_BINARY_DIR}/include/qleany/qleany_export.h"
)

target_link_libraries(qleany PUBLIC Qt6::Core Qt6::Concurrent Qt6::Sql)

if(QLEANY_BUILD_WITH_QT_GUI)
  target_link_libraries(qleany PRIVATE Qt::Gui)
endif()

# state that Qleany need PIC when the default is shared libraries
set_target_properties(qleany PROPERTIES POSITION_INDEPENDENT_CODE
                                        ${BUILD_SHARED_LIBS})

# configure the install location
message(STATUS "Configuring install location")
include(GNUInstallDirs)

# Install library
install(
  TARGETS qleany
  EXPORT qleanyTargets
  LIBRARY DESTINATION lib
  ARCHIVE DESTINATION lib
  PUBLIC_HEADER DESTINATION include/qleany)

# Install public headers
install(DIRECTORY include/qleany DESTINATION include)

# install license
install(FILES LICENSE DESTINATION share/qleany)

# Install the export set for use with the install-tree
install(
  EXPORT qleanyTargets
  FILE qleanyTargets.cmake
  DESTINATION lib/cmake/qleany)

# Create and install qleanyConfig.cmake
message(STATUS "Creating and installing qleanyConfig.cmake")
include(CMakePackageConfigHelpers)
write_basic_package_version_file(
  qleanyConfigVersion.cmake
  VERSION ${PROJECT_VERSION}
  COMPATIBILITY SameMajorVersion)

configure_package_config_file(
  ${CMAKE_CURRENT_SOURCE_DIR}/qleanyConfig.cmake.in
  ${CMAKE_CURRENT_BINARY_DIR}/qleanyConfig.cmake
  INSTALL_DESTINATION lib/cmake/qleany)

install(FILES ${CMAKE_CURRENT_BINARY_DIR}/qleanyConfig.cmake
              ${CMAKE_CURRENT_BINARY_DIR}/qleanyConfigVersion.cmake
        DESTINATION lib/cmake/qleany)

# Export the targets to a script message(STATUS "Exporting targets to a script")
# export(EXPORT ${PROJECT_NAME}Targets FILE
# ${CMAKE_CURRENT_BINARY_DIR}/qleanyTargets.cmake)

# Subdirectories that contain CMakeLists.txt for the framework tests and example
if(QLEANY_BUILD_TESTS)
  add_subdirectory(tests)
endif()

if(QLEANY_BUILD_EXAMPLES)
  include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
  include_directories(${CMAKE_BINARY_DIR}/include)
  set(CMAKE_PREFIX_PATH ${CMAKE_BINARY_DIR} ${CMAKE_PREFIX_PATH})
  set(CMAKE_LIBRARY_PATH ${CMAKE_BINARY_DIR} ${CMAKE_LIBRARY_PATH})

  add_subdirectory(examples)
endif()
