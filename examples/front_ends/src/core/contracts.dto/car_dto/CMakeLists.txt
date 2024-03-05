# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME front-ends-example-contracts-dto-car)

set(SRC

    
    car/car_dto.h
    car/car_with_details_dto.h
    car/create_car_dto.h
    car/update_car_dto.h
    car/car_relation_dto.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

set_target_properties(${LIBRARY_NAME} PROPERTIES
                    POSITION_INDEPENDENT_CODE ${BUILD_SHARED_LIBS}
                    )

add_dependencies(${LIBRARY_NAME} front-ends-example-contracts-dto-brand)
add_dependencies(${LIBRARY_NAME} front-ends-example-contracts-dto-passenger)

target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-contracts-dto-brand)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-contracts-dto-passenger)


target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)