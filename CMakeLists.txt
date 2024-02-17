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

# Create a library target for the Qleany framework
set(LIB_PUBLIC_HEADERS
    include/qleany/tools/undo_redo/alter_command.h
    include/qleany/tools/undo_redo/query_command.h
    include/qleany/tools/undo_redo/undo_redo_stack.h
    include/qleany/tools/undo_redo/threaded_undo_redo_system.h
    include/qleany/tools/undo_redo/undo_redo_system.h
    include/qleany/tools/undo_redo/undo_redo_command.h
    include/qleany/tools/undo_redo/undo_redo_scopes.h
    include/qleany/tools/automapper/automapper.h
    include/qleany/common/error.h
    include/qleany/common/result.h
    include/qleany/database/types.h
    include/qleany/database/database_context.h
    include/qleany/database/database_table_group.h
    include/qleany/database/tools.h
    include/qleany/database/one_to_one_associator.h
    include/qleany/database/one_to_many_unordered_associator.h
    include/qleany/database/one_to_many_ordered_associator.h
    include/qleany/immutable_database/types.h
    include/qleany/immutable_database/database_context.h
    include/qleany/immutable_database/database_table_group.h
    include/qleany/immutable_database/tools.h
    include/qleany/immutable_database/one_to_one_associator.h
    include/qleany/immutable_database/one_to_many_unordered_associator.h
    include/qleany/immutable_database/one_to_many_ordered_associator.h
    include/qleany/qleany_global.h
    include/qleany/repository/generic_repository.h
    include/qleany/repository/repository_provider.h
    include/qleany/domain/entity_schema.h
    include/qleany/domain/entity_base.h
    include/qleany/contracts/repository/interface_repository_provider.h
    include/qleany/contracts/repository/interface_generic_repository.h
    include/qleany/contracts/repository/interface_repository.h
    include/qleany/contracts/repository/signal_holder.h
    include/qleany/contracts/database/interface_database_table_group.h
    include/qleany/contracts/database/interface_database_context.h
    include/qleany/contracts/immutable_database/interface_database_table_group.h
    include/qleany/contracts/immutable_database/interface_database_context.h
)

# configure the library
file(GLOB_RECURSE SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB_RECURSE PUBLIC_HEADERS "${CMAKE_CURRENT_SOURCE_DIR}/include/${PROJECT_NAME_LOWER}/*.h")

message(STATUS "PUBLIC_HEADERS: ${PUBLIC_HEADERS}")

# Create the library target
add_library(Qleany ${LIB_TYPE}
    ${SOURCES}
    ${PUBLIC_HEADERS}
)

target_include_directories(${PROJECT_NAME} PUBLIC
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}/qleany>
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
)

set_target_properties(Qleany PROPERTIES CXX_STANDARD 20)

set_target_properties(Qleany PROPERTIES OUTPUT_NAME ${PROJECT_NAME_LOWER})
set_target_properties(${PROJECT_NAME} PROPERTIES PUBLIC_HEADER ${PUBLIC_HEADERS})

target_link_libraries(Qleany PUBLIC Qt6::Core Qt6::Concurrent Qt6::Sql)

if(QLEANY_BUILD_WITH_QT_GUI)
    target_link_libraries(Qleany PRIVATE Qt::Gui)
endif()

target_compile_definitions(Qleany PRIVATE QLEANY_LIBRARY)


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


# Create and install keygen-licenserConfig.cmake
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

# Install the export set for use with the install-tree
install(EXPORT ${PROJECT_NAME}Targets
    FILE ${PROJECT_NAME_LOWER}Targets.cmake
    NAMESPACE ${PROJECT_NAME}::
    DESTINATION lib/cmake/${PROJECT_NAME_LOWER}
)

# Subdirectories that contain CMakeLists.txt for the framework tests and example
if(QLEANY_BUILD_TESTS)
    add_subdirectory(tests)
endif()

if(QLEANY_BUILD_EXAMPLES)
    add_subdirectory(examples)
endif()
