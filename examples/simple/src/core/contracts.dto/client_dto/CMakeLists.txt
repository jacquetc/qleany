# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME simple-example-contracts-dto-client)

set(SRC

    
    client/client_dto.h
    client/client_with_details_dto.h
    client/create_client_dto.h
    client/update_client_dto.h
    client/client_relation_dto.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

set_target_properties(${LIBRARY_NAME} PROPERTIES
                    POSITION_INDEPENDENT_CODE ${BUILD_SHARED_LIBS}
                    )

add_dependencies(${LIBRARY_NAME} simple-example-contracts-dto-passenger)

target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts-dto-passenger)


target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)