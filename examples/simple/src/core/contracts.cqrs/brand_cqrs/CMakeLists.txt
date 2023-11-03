# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME simple-example-contracts-cqrs-brand)

set(SRC

    
    brand/queries/get_brand_query.h
    brand/commands/create_brand_command.h
    brand/commands/update_brand_command.h
    brand/commands/remove_brand_command.h
    brand/validators/create_brand_command_validator.h
    brand/validators/update_brand_command_validator.h
    brand/validators/remove_brand_command_validator.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-domain)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts-dto-brand)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)