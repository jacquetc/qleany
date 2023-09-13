# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME simple-example-contracts-dto-car)

set(SRC

    
    car/car_dto.h
    car/car_with_details_dto.h
    car/create_car_dto.h
    car/update_car_dto.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")


add_dependencies(${LIBRARY_NAME} simple-example-contracts-dto-passenger)

add_dependencies(${LIBRARY_NAME} simple-example-contracts-dto-brand)


target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts-dto-passenger)

target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts-dto-brand)


target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)