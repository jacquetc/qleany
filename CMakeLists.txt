cmake_minimum_required(VERSION 3.18)
project(Qleany)

# Compiler settings
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(QT_MIN_VERSION "6.4.0")
set(QT_DEFAULT_MAJOR_VERSION 6)
set(CMAKE_AUTOMOC ON)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake/ ${CMAKE_SOURCE_DIR}/cmake/3rdparty/ ${CMAKE_MODULE_PATH})

# options
option(QLEANY_BUILD_EXAMPLES "Build examples" ON)
option(QLEANY_BUILD_TESTS "Build tests" ON)
option(BUILD_SHARED_LIBS "Build shared libraries" OFF)
option(QLEANY_BUILD_WITH_QT_GUI "Build with Qt GUI" ON)

if(QLEANY_BUILD_WITH_QT_GUI)
    add_compile_definitions(QLEANY_BUILD_WITH_QT_GUI=1)
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
    include/qleany/contracts/database/interface_foreign_entity.h
    include/qleany/contracts/database/interface_database_context.h
)

# Create the library target

add_library(Qleany ${LIB_TYPE}
    ${LIB_PUBLIC_HEADERS}
        src/tools/undo_redo/query_command.cpp
        src/tools/undo_redo/undo_redo_stack.cpp
        src/tools/undo_redo/threaded_undo_redo_system.cpp
        src/tools/undo_redo/undo_redo_system.cpp
        src/tools/undo_redo/undo_redo_command.cpp
        src/tools/automapper/automapper.cpp
        src/tools/undo_redo/undo_redo_scopes.cpp
        src/repository/repository_provider.cpp
        src/database/database_context.cpp
        src/domain/entity_base.cpp
        src/contracts/repository/signal_holder.cpp
)
target_include_directories(Qleany PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include> # <prefix>/include
)
target_link_libraries(Qleany PUBLIC Qt6::Core Qt6::Concurrent Qt6::Sql)

if(QLEANY_BUILD_WITH_QT_GUI)
    target_link_libraries(Qleany PRIVATE Qt::Gui)
endif()

target_compile_definitions(Qleany PRIVATE QLEANY_LIBRARY)

# Specify where to install the library and headers

install(TARGETS Qleany
    EXPORT QleanyTargets
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
    INCLUDES DESTINATION include
)

install(EXPORT QleanyTargets
    FILE QleanyTargets.cmake
    NAMESPACE Qleany::
    DESTINATION lib/cmake/Qleany
)

install(DIRECTORY include/
    DESTINATION include
)







# Subdirectories that contain CMakeLists.txt for the framework tests and example
if(QLEANY_BUILD_TESTS)
    add_subdirectory(tests)
endif()
if(QLEANY_BUILD_EXAMPLES)
    add_subdirectory(examples)
endif()

