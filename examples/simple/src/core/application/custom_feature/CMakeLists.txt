# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME simple-example-application-feature-custom)

set(SRC

    
    custom/commands/write_random_things_command_handler.h
    custom/commands/write_random_things_command_handler.cpp
    custom/commands/close_system_command_handler.h
    custom/commands/close_system_command_handler.cpp
    custom/queries/get_current_time_query_handler.h
    custom/queries/get_current_time_query_handler.cpp
    application_custom_export.h
)
qt_add_library(${LIBRARY_NAME} SHARED ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${LIBRARY_NAME} PUBLIC SIMPLEEXAMPLE_APPLICATION_CUSTOM_LIBRARY)

target_link_libraries(${LIBRARY_NAME} PUBLIC Qleany)

target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts)
target_link_libraries(${LIBRARY_NAME} PUBLIC simple-example-contracts-cqrs-custom)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)

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
