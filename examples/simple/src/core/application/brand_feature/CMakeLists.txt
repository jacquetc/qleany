# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME simple-example-application-feature-brand)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if(BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else()
    set(LIB_TYPE STATIC)
endif()

set(SRC

    
    brand/commands/create_brand_command_handler.cpp
    brand/commands/create_brand_command_handler.h
    brand/commands/remove_brand_command_handler.cpp
    brand/commands/remove_brand_command_handler.h
    brand/commands/update_brand_command_handler.cpp
    brand/commands/update_brand_command_handler.h
    brand/queries/get_brand_query_handler.cpp
    brand/queries/get_brand_query_handler.h
    brand/queries/get_all_brand_query_handler.cpp
    brand/queries/get_all_brand_query_handler.h
    simple_example_application_brand_export.h
)
qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC 
${CMAKE_CURRENT_SOURCE_DIR}
${CMAKE_CURRENT_BINARY_DIR}
)

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
    BASE_NAME simple_example_application_brand)

target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-domain)
target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-persistence)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts-cqrs-brand)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
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