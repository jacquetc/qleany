# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core Sql REQUIRED)

set(LIBRARY_NAME simple-example-contracts)

include(repository_interfaces.cmake)

set(SRCS
    contracts_export.h
)

qt_add_library(${LIBRARY_NAME} STATIC ${SRCS} ${REPOSITORY_LIST})

target_compile_definitions(${LIBRARY_NAME} PRIVATE SIMPLEEXAMPLE_CONTRACTS_LIBRARY)

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-domain)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core Qt6::Sql
)