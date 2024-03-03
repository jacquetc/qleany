# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.
find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core Qml)

set(PLUGIN_NAME simple-example-qml-interactors)

qt_add_library(${PLUGIN_NAME} STATIC)

qt6_add_qml_module(${PLUGIN_NAME}
    URI "Interactors"
    VERSION 1.0
    SOURCES
    
    foreign_car_interactor.h
    foreign_brand_interactor.h
    foreign_passenger_interactor.h
    foreign_client_interactor.h
    foreign_custom_interactor.h
    foreign_event_dispatcher.h
    foreign_undo_redo_interactor.h

)

target_link_libraries(${PLUGIN_NAME} PRIVATE simple-example-interactor)
target_link_libraries(${PLUGIN_NAME} PRIVATE QCoro::Core QCoro::Qml)

target_link_libraries(${PLUGIN_NAME}
    PRIVATE Qt6::Quick Qt6::Core Qt6::Qml
)