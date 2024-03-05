project(SimpleExampleDesktopApp LANGUAGES CXX)

set(APP_NAME simple-example-desktop-application)

find_package(Qt6 COMPONENTS Core Gui Widgets Concurrent Sql REQUIRED)

find_package(QCoro6 REQUIRED COMPONENTS Core)
# find_package(qleany REQUIRED)
# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

if (Qt6_VERSION VERSION_GREATER_EQUAL 6.3)
    qt_standard_project_setup()
endif()

# Create an executable target for the example project
add_executable(${APP_NAME}
    main.cpp
    mainwindow.h mainwindow.cpp mainwindow.ui
    # add other source files for the example
)
add_dependencies(${APP_NAME} Qleany)

target_link_libraries(${APP_NAME} PUBLIC Qt6::Core Qt6::Widgets Qt6::Concurrent Qt6::Sql)
# Link the example against the Qleany library
target_link_libraries(${APP_NAME} PUBLIC Qleany)
target_link_libraries(${APP_NAME} PUBLIC simple-example-entities)
target_link_libraries(${APP_NAME} PUBLIC simple-example-persistence)
target_link_libraries(${APP_NAME} PUBLIC simple-example-interactor)
target_link_libraries(${APP_NAME} PUBLIC simple-example-presenter)
target_link_libraries(${APP_NAME} PRIVATE QCoro::Core)
