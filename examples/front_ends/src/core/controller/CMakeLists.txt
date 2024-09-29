# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core Gui REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core)

set(LIBRARY_NAME front-ends-example-controller)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if(BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else()
    set(LIB_TYPE STATIC)
endif()


include(controllers.cmake)

set(SRCS
    front_ends_example_controller_export.h
    event_dispatcher.h
    event_dispatcher.cpp
    controller_registration.h
    error_signals.h
    progress_signals.h
    controller_registration.cpp  
    undo_redo/query_command.cpp
    undo_redo/query_command.h
    undo_redo/threaded_undo_redo_system.cpp
    undo_redo/threaded_undo_redo_system.h
    undo_redo/undo_redo_command.cpp
    undo_redo/undo_redo_command.h
    undo_redo/undo_redo_scopes.cpp
    undo_redo/undo_redo_scopes.h
    undo_redo/undo_redo_stack.cpp
    undo_redo/undo_redo_stack.h
    undo_redo/undo_redo_system.cpp
    undo_redo/undo_redo_system.h
)

qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRCS} ${CONTROLLER_LIST})

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
    BASE_NAME front_ends_example_controller)

target_include_directories(${LIBRARY_NAME} PUBLIC 
${CMAKE_CURRENT_SOURCE_DIR}
${CMAKE_CURRENT_BINARY_DIR}

)

target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-common)
target_link_libraries(${LIBRARY_NAME} PRIVATE front-ends-example-entities)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-application-feature-car)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-application-feature-brand)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-application-feature-passenger)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-application-feature-client)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-application-feature-custom)
target_link_libraries(${LIBRARY_NAME} PRIVATE
    Qt6::Core Qt6::Gui)
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