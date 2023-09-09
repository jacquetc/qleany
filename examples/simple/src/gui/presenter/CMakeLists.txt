
set(LIBRARY_NAME simple-example-presenter)

set(SRC
    presenter_export.h
    passenger_list_model.cpp
    passenger_list_model.h

)
qt_add_library(${LIBRARY_NAME} SHARED ${SRC}
    single_passenger.h single_passenger.cpp)

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${LIBRARY_NAME} PRIVATE SIMPLEEXAMPLE_PRESENTER_LIBRARY)

add_dependencies(${LIBRARY_NAME} simple-example-controller)

target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-controller)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core Qt6::Gui
)
target_link_libraries(${LIBRARY_NAME} PRIVATE QCoro::Core)

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
