find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)

set(PLUGIN_NAME controllers)

qt_add_library(${PLUGIN_NAME} STATIC)

qt6_add_qml_module(${PLUGIN_NAME}
    URI "Controllers"
    VERSION 1.0
    RESOURCE_PREFIX "/controllers"
    SOURCES
    
    foreign_passenger_controller.h
    foreign_car_controller.h
    SOURCES foreign_event_dispatcher.h

)

target_link_libraries(${PLUGIN_NAME} PRIVATE simple-example-controller)
target_link_libraries(${PLUGIN_NAME} PRIVATE QCoro::Core)

target_link_libraries(${PLUGIN_NAME}
    PRIVATE Qt6::Quick Qt6::Core Qt6::Qml
)
