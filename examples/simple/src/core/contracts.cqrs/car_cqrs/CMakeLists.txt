# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME simple-example-contracts-cqrs-car)

set(SRC

    
    car/queries/get_car_query.h
    car/commands/create_car_command.h
    car/commands/update_car_command.h
    car/commands/remove_car_command.h
    car/validators/create_car_command_validator.h
    car/validators/update_car_command_validator.h
    car/validators/remove_car_command_validator.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-entities)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts-dto-car)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)
