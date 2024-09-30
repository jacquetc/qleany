# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME front-ends-example-application-feature-car)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if(BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else()
    set(LIB_TYPE STATIC)
endif()

set(SRC

    
    car/commands/create_car_command_handler.cpp
    car/commands/create_car_command_handler.h
    car/commands/remove_car_command_handler.cpp
    car/commands/remove_car_command_handler.h
    car/commands/update_car_command_handler.cpp
    car/commands/update_car_command_handler.h
    car/queries/get_car_query_handler.cpp
    car/queries/get_car_query_handler.h
    car/queries/get_all_car_query_handler.cpp
    car/queries/get_all_car_query_handler.h
    car/queries/get_car_with_details_query_handler.cpp
    car/queries/get_car_with_details_query_handler.h
)
qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC 
${CMAKE_CURRENT_SOURCE_DIR}
${CMAKE_CURRENT_BINARY_DIR}
)

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
    BASE_NAME front_ends_example_application_car)

target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-common)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-entities)
target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-persistence)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-contracts-cqrs-car)

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