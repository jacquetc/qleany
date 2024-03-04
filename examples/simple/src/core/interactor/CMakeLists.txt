# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core Gui REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core)
# find_package(qleany REQUIRED) # CONFIG disabled for now, because it's not available in the qleany package


set(LIBRARY_NAME simple-example-interactor)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if(BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else()
    set(LIB_TYPE STATIC)
endif()


include(interactors.cmake)

set(SRCS
    event_dispatcher.h
    event_dispatcher.cpp
    interactor_registration.h
    error_signals.h
    progress_signals.h
    interactor_registration.cpp
)

qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRCS} ${INTERACTOR_LIST})

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
    BASE_NAME simple_example_interactor)

target_include_directories(${LIBRARY_NAME} PUBLIC 
${CMAKE_CURRENT_SOURCE_DIR}
${CMAKE_CURRENT_BINARY_DIR}
)

target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-entities)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-application-feature-car)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-application-feature-brand)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-application-feature-passenger)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-application-feature-client)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-application-feature-custom)
target_link_libraries(${LIBRARY_NAME} PRIVATE
    Qt6::Core Qt6::Gui)
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
