# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME simple-example-application-feature-car)

set(SRC

    
    car/commands/create_car_command_handler.cpp
    car/commands/create_car_command_handler.h
    car/commands/remove_car_command_handler.cpp
    car/commands/remove_car_command_handler.h
    car/commands/remove_car_tree_command_handler.cpp
    car/commands/remove_car_tree_command_handler.h
    car/commands/update_car_command_handler.cpp
    car/commands/update_car_command_handler.h
    car/queries/get_car_query_handler.cpp
    car/queries/get_car_query_handler.h
    car/queries/get_all_car_query_handler.cpp
    car/queries/get_all_car_query_handler.h
    car/queries/get_car_with_details_query_handler.cpp
    car/queries/get_car_with_details_query_handler.h
    application_car_export.h
)
qt_add_library(${LIBRARY_NAME} SHARED ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${LIBRARY_NAME} PRIVATE SIMPLEEXAMPLE_APPLICATION_CAR_LIBRARY)
target_link_libraries(${LIBRARY_NAME} PUBLIC Qleany)

target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts-cqrs-car)

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
