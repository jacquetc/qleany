
set(LIBRARY_NAME simple-example-controller)

set(SRC
    passenger/passenger_controller.cpp
    passenger/passenger_controller.h
    passenger/passenger_signals.h
    car/car_controller.cpp
    car/car_controller.h
    car/car_signals.h
)
qt_add_library(${LIBRARY_NAME} SHARED ${SRC}
    event_dispatcher.h
    event_dispatcher.cpp
    controller_registration.h
    error_signals.h
    controller_export.h
    controller_registration.cpp
)

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${LIBRARY_NAME} PRIVATE SIMPLEEXAMPLE_CONTROLLER_LIBRARY)

#target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-application-feature-brand)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-application-feature-passenger)
#target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-application-feature-client)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-application-feature-car)

target_link_libraries(${LIBRARY_NAME} PRIVATE
    Qt6::Core Qt::Gui)
target_link_libraries(${LIBRARY_NAME} PRIVATE QCoro::Core)
target_link_libraries(${LIBRARY_NAME} PRIVATE Qleany)

if(IOS)
    install(TARGETS ${LIBRARY_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS ${LIBRARY_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS ${LIBRARY_NAME}
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
