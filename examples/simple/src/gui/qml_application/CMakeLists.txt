cmake_minimum_required(VERSION 3.21.1)

option(LINK_INSIGHT "Link Qt Insight Tracker library" ON)
option(BUILD_QDS_COMPONENTS "Build design studio components" OFF)

project(SimpleExampleQMLApp LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)


# To make the mocks appear in Qt Designer, you need to restart Qt Designer after running cmake file generation


find_package(Qt6 COMPONENTS Core Gui Qml Quick Concurrent)
find_package(QCoro6 REQUIRED COMPONENTS Core Qml)
# find_package(qleany CONFIG REQUIRED)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

if (Qt6_VERSION VERSION_GREATER_EQUAL 6.3)
    qt_standard_project_setup()
endif()

set(APP_NAME SimpleExampleQMLApp)
qt_add_executable(${APP_NAME}  src/main.cpp)

qt_add_resources(${APP_NAME}  "configuration"
    PREFIX "/"
    FILES
        qtquickcontrols2.conf
)

add_dependencies(${APP_NAME} Qleany)

# Link the example against the Qleany library
target_link_libraries(${APP_NAME} PUBLIC Qleany)
target_link_libraries(${APP_NAME} PUBLIC simple-example-entities)
target_link_libraries(${APP_NAME} PUBLIC simple-example-persistence)
target_link_libraries(${APP_NAME} PUBLIC simple-example-interactor)
target_link_libraries(${APP_NAME} PUBLIC simple-example-presenter)
target_link_libraries(${APP_NAME} PRIVATE QCoro::Core QCoro::Qml)

target_link_libraries(${APP_NAME} PRIVATE
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Quick
    Qt${QT_VERSION_MAJOR}::Qml
    Qt${QT_VERSION_MAJOR}::Concurrent
)

if (BUILD_QDS_COMPONENTS)
    include(${CMAKE_CURRENT_SOURCE_DIR}/qmlcomponents)
endif()

include(${CMAKE_CURRENT_SOURCE_DIR}/qmlmodules) # do not rename this file, it is generated and used by Qt Designer
include(${CMAKE_CURRENT_SOURCE_DIR}/realqmlmodules.cmake)

if (LINK_INSIGHT)
    include(${CMAKE_CURRENT_SOURCE_DIR}/insight)
endif ()

include(GNUInstallDirs)
install(TARGETS ${APP_NAME}
  BUNDLE DESTINATION .
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
