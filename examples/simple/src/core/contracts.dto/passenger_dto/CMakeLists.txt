# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME simple-example-contracts-dto-passenger)

set(SRC

    
    passenger/passenger_dto.h
    passenger/create_passenger_dto.h
    passenger/update_passenger_dto.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})


set_target_properties(${LIBRARY_NAME} PROPERTIES
                      POSITION_INDEPENDENT_CODE ${BUILD_SHARED_LIBS}
                      )

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")




target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)
