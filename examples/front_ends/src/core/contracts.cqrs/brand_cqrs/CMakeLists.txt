# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME front-ends-example-contracts-cqrs-brand)

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

set_target_properties(${LIBRARY_NAME} PROPERTIES
                    POSITION_INDEPENDENT_CODE ${BUILD_SHARED_LIBS}
                    )

set_target_properties(${LIBRARY_NAME} PROPERTIES LINKER_LANGUAGE CXX)
        
target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-entities)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-contracts-dto-brand)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)