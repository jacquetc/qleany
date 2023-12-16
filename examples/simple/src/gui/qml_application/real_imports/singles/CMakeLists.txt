find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)

set(PLUGIN_NAME simple-example-qml-singles)

qt_add_library(${PLUGIN_NAME} STATIC)
qt6_add_qml_module(${PLUGIN_NAME}
    URI "Singles"
    VERSION 1.0
    RESOURCE_PREFIX "/singles"
    SOURCES
    foreign_single_car.h
    
    

)

target_link_libraries(${PLUGIN_NAME} PRIVATE simple-example-presenter)

target_link_libraries(${PLUGIN_NAME}
    PRIVATE Qt6::Core Qt6::Qml Qt6::Quick
)
