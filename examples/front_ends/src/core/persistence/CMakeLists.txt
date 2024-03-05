# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core Sql REQUIRED)
# find_package(qleany CONFIG REQUIRED)

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

target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-entities)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core Qt6::Sql
)
target_link_libraries(${LIBRARY_NAME} PRIVATE Qleany)

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