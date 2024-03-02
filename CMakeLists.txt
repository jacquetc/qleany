cmake_minimum_required(VERSION 3.18)


list(APPEND CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake)

# get version from git

include(VersionFromGit)
version_from_git(
LOG       ON
)
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



project(Qleany VERSION ${QLEANY_VERSION} LANGUAGES CXX DESCRIPTION "Qleany is a light framework for Clean Architecture with Qt6")
string(TOLOWER ${PROJECT_NAME} PROJECT_NAME_LOWER)

# Compiler settings
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(QT_MIN_VERSION "6.4.0")
set(QT_DEFAULT_MAJOR_VERSION 6)
set(CMAKE_AUTOMOC ON)

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
find_package(Qt6 COMPONENTS Core Sql Concurrent REQUIRED)
qt_standard_project_setup()

if(QLEANY_BUILD_WITH_QT_GUI)
    find_package(Qt6 COMPONENTS Gui REQUIRED)
endif()

if(QLEANY_BUILD_EXAMPLES)
    find_package(Qt6 COMPONENTS Widgets REQUIRED)
endif()

find_package(QCoro6 REQUIRED COMPONENTS Core)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

# configure the library
file(GLOB_RECURSE SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB_RECURSE PUBLIC_HEADERS "${CMAKE_CURRENT_SOURCE_DIR}/include/${PROJECT_NAME_LOWER}/*.h")

message(STATUS "PUBLIC_HEADERS: ${PUBLIC_HEADERS}")

# Create the library target
add_library(Qleany ${LIB_TYPE}
    ${SOURCES}
    ${PUBLIC_HEADERS}
)

# Generate export header
message(STATUS "Generating export header")

include(GenerateExportHeader)
generate_export_header(Qleany
    EXPORT_FILE_NAME ${CMAKE_CURRENT_BINARY_DIR}/include/qleany/qleany_export.h
)

target_include_directories(${PROJECT_NAME} PRIVATE
    ${CMAKE_CURRENT_BINARY_DIR}/include
)

target_include_directories(${PROJECT_NAME} PUBLIC
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}/qleany>
    $<BUILD_INTERFACE:${CMAKE_CURRENT_BINARY_DIR}/include>
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
)

set_target_properties(Qleany PROPERTIES CXX_STANDARD 20)

set_target_properties(Qleany PROPERTIES OUTPUT_NAME ${PROJECT_NAME_LOWER})
set_target_properties(${PROJECT_NAME} PROPERTIES PUBLIC_HEADER "${PUBLIC_HEADERS};${CMAKE_CURRENT_BINARY_DIR}/include/qleany/qleany_export.h")

target_link_libraries(Qleany PUBLIC Qt6::Core Qt6::Concurrent Qt6::Sql)

if(QLEANY_BUILD_WITH_QT_GUI)
    target_link_libraries(Qleany PRIVATE Qt::Gui)
endif()

# state that Qleany need PIC when the default is shared libraries
set_target_properties(Qleany PROPERTIES
                      POSITION_INDEPENDENT_CODE ${BUILD_SHARED_LIBS}
                      )


# configure the install location
message(STATUS "Configuring install location")
include(GNUInstallDirs)

# Install library
install(TARGETS ${PROJECT_NAME}
        EXPORT ${PROJECT_NAME}Targets
        LIBRARY DESTINATION lib
        ARCHIVE DESTINATION lib
        PUBLIC_HEADER  DESTINATION include/qleany
)

# Install public headers
install(DIRECTORY include/${PROJECT_NAME_LOWER} DESTINATION include)


# install license
install(FILES LICENSE DESTINATION share/${PROJECT_NAME_LOWER})


# Install the export set for use with the install-tree
install(EXPORT ${PROJECT_NAME}Targets
    FILE ${PROJECT_NAME_LOWER}Targets.cmake
    DESTINATION lib/cmake/${PROJECT_NAME_LOWER}
)  

# Create and install qleanyConfig.cmake
message(STATUS "Creating and installing qleanyConfig.cmake")
include(CMakePackageConfigHelpers)
write_basic_package_version_file(
    ${PROJECT_NAME_LOWER}ConfigVersion.cmake
    VERSION ${PROJECT_VERSION}
    COMPATIBILITY SameMajorVersion)

configure_package_config_file(
    ${CMAKE_CURRENT_SOURCE_DIR}/${PROJECT_NAME_LOWER}Config.cmake.in
    ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME_LOWER}Config.cmake
    INSTALL_DESTINATION lib/cmake/${PROJECT_NAME_LOWER})

install(FILES
        ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME_LOWER}Config.cmake
        ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME_LOWER}ConfigVersion.cmake
        DESTINATION lib/cmake/${PROJECT_NAME_LOWER})

# Export the targets to a script
 # message(STATUS "Exporting targets to a script")
 # export(EXPORT ${PROJECT_NAME}Targets
 #       FILE ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME_LOWER}Targets.cmake)


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
