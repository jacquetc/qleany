find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)

set(PLUGIN_NAME simple-example-models)

qt_add_library(${PLUGIN_NAME} STATIC)
qt6_add_qml_module(${PLUGIN_NAME}
    URI "Models"
    VERSION 1.0
    RESOURCE_PREFIX "/models"
    SOURCES
    foreign_car_list_model.h
    foreign_passenger_list_model_from_car_passengers.h

    

)

target_link_libraries(${PLUGIN_NAME} PRIVATE simple-example-presenter)

target_link_libraries(${PLUGIN_NAME}
    PRIVATE Qt6::Core Qt6::Qml Qt6::Quick
)
