# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME simple-example-application-feature-passenger)

set(SRC

    
    passenger/commands/create_passenger_command_handler.cpp
    passenger/commands/create_passenger_command_handler.h
    passenger/commands/remove_passenger_command_handler.cpp
    passenger/commands/remove_passenger_command_handler.h
    passenger/commands/update_passenger_command_handler.cpp
    passenger/commands/update_passenger_command_handler.h
    passenger/queries/get_passenger_query_handler.cpp
    passenger/queries/get_passenger_query_handler.h
    passenger/queries/get_all_passenger_query_handler.cpp
    passenger/queries/get_all_passenger_query_handler.h
    application_passenger_export.h
)
qt_add_library(${LIBRARY_NAME} SHARED ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${LIBRARY_NAME} PUBLIC SIMPLEEXAMPLE_APPLICATION_PASSENGER_LIBRARY)

add_dependencies(${LIBRARY_NAME} simple-example-contracts)
add_dependencies(${LIBRARY_NAME} simple-example-contracts-cqrs-passenger)

target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts-cqrs-passenger)

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