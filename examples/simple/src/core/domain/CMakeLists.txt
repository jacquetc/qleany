# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core REQUIRED)

set(LIBRARY_NAME simple-example-domain)

include(entities.cmake)

set(SRCS
    domain_export.h
    entities.h
    domain_registration.h
)

qt_add_library(${LIBRARY_NAME} STATIC ${SRCS} ${ENTITY_LIST})

target_compile_definitions(${LIBRARY_NAME} PRIVATE SIMPLEEXAMPLE_DOMAIN_LIBRARY)

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(${LIBRARY_NAME} PRIVATE Qt6::Core )
target_link_libraries(${LIBRARY_NAME} PUBLIC Qleany)