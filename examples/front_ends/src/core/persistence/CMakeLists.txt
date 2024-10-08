# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core Sql REQUIRED)

set(LIBRARY_NAME front-ends-example-persistence)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if(BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else()
    set(LIB_TYPE STATIC)
endif()

include(repositories.cmake)

set(SRCS
    persistence_registration.h
    persistence_registration.cpp
    database/database_context.h
    database/database_context.cpp
    database/database_table_group.h
    database/many_to_many_unordered_associator.h
    database/one_to_many_unordered_associator.h
    database/one_to_many_ordered_associator.h
    database/one_to_one_associator.h
    database/tools.h
)

qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRCS} ${REPOSITORY_LIST})

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
BASE_NAME front_ends_example_persistence
)

target_include_directories(${LIBRARY_NAME} PUBLIC 
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_BINARY_DIR}
)

target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-common)
target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-entities)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core Qt6::Sql
)

if(IOS)
    install(TARGETS ${LIBRARY_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS ${LIBRARY_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS ${LIBRARY_NAME}
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")