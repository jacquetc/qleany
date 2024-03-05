# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME front-ends-example-contracts-cqrs-passenger)

set(SRC

    
    passenger/queries/get_passenger_query.h
    passenger/commands/create_passenger_command.h
    passenger/commands/update_passenger_command.h
    passenger/commands/remove_passenger_command.h
    passenger/validators/create_passenger_command_validator.h
    passenger/validators/update_passenger_command_validator.h
    passenger/validators/remove_passenger_command_validator.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

set_target_properties(${LIBRARY_NAME} PROPERTIES
                    POSITION_INDEPENDENT_CODE ${BUILD_SHARED_LIBS}
                    )
                    
target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-entities)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-contracts-dto-passenger)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)