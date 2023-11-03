# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core REQUIRED)

set(LIBRARY_NAME simple-example-presenter)

include(presenters.cmake)

set(SRCS
    presenter_export.h
)

qt_add_library(${LIBRARY_NAME} SHARED ${SRCS} ${PRESENTER_LIST})

target_compile_definitions(${LIBRARY_NAME} PRIVATE SIMPLEEXAMPLE_PRESENTER_LIBRARY)

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")


target_link_libraries(${LIBRARY_NAME} PRIVATE
    Qt6::Core Qt::Gui)
target_link_libraries(${LIBRARY_NAME} PRIVATE QCoro::Core)
target_link_libraries(${LIBRARY_NAME} PUBLIC Qleany)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-controller)

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