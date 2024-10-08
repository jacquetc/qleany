# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core Gui REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core)

set(LIBRARY_NAME front-ends-example-presenter)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if(BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else()
    set(LIB_TYPE STATIC)
endif()

include(presenters.cmake)

qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${PRESENTER_LIST})

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
    BASE_NAME front_ends_example_presenter)

target_include_directories(${LIBRARY_NAME} PUBLIC 
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_BINARY_DIR}
)

target_link_libraries(${LIBRARY_NAME} PRIVATE
    Qt6::Core Qt6::Gui)
target_link_libraries(${LIBRARY_NAME} PRIVATE QCoro::Core)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-common)
target_link_libraries(${LIBRARY_NAME} PUBLIC front-ends-example-controller)

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