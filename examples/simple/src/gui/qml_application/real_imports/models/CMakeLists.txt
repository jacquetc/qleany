# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.
find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core Qml)

set(PLUGIN_NAME simple-example-qml-models)

qt_add_library(${PLUGIN_NAME} STATIC)

qt6_add_qml_module(${PLUGIN_NAME}
    URI "Models"
    VERSION 1.0
    RESOURCE_PREFIX "/models"
    SOURCES
    
    foreign_passenger_list_model_from_car_passengers_model.h
    foreign_car_list_model_model.h

)

target_link_libraries(${PLUGIN_NAME} PRIVATE simple-example-presenter)
target_link_libraries(${PLUGIN_NAME} PRIVATE QCoro::Core QCoro::Qml)

target_link_libraries(${PLUGIN_NAME}
    PRIVATE Qt6::Quick Qt6::Core Qt6::Qml
)