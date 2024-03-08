# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME simple-example-contracts-cqrs-custom)

set(SRC

    
    custom/queries/get_current_time_query.h
    custom/commands/write_random_things_command.h
    custom/commands/run_long_operation_command.h
    custom/commands/close_system_command.h
    custom/validators/write_random_things_command_validator.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

set_target_properties(${LIBRARY_NAME} PROPERTIES
                    POSITION_INDEPENDENT_CODE ${BUILD_SHARED_LIBS}
                    )

set_target_properties(${LIBRARY_NAME} PROPERTIES LINKER_LANGUAGE CXX)
        
target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-entities)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts-dto-custom)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)