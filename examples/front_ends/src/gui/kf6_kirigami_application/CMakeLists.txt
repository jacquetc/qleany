# SPDX-License-Identifier: BSD-2-Clause
# SPDX-FileCopyrightText: %{CURRENT_YEAR} %{AUTHOR} <%{EMAIL}>

option(BUILD_WITH_MOCKS "Build with mocks" OFF)

set(APP_NAME frontendsexample_static)

# Target: static library
qt_add_qml_module(${APP_NAME}
    STATIC
    URI org.kde.frontendsexample
    VERSION 1.0
    QML_FILES
    contents/ui/Main.qml
    contents/ui/About.qml
)

target_sources(${APP_NAME} PUBLIC
    app.cpp
)
qcoro_enable_coroutines()

target_link_libraries(${APP_NAME} PUBLIC front-ends-example-entities)
target_link_libraries(${APP_NAME} PUBLIC front-ends-example-persistence)
target_link_libraries(${APP_NAME} PUBLIC front-ends-example-controller)
target_link_libraries(${APP_NAME} PUBLIC front-ends-example-presenter)
target_link_libraries(${APP_NAME} PRIVATE QCoro::Core QCoro::Qml)

target_link_libraries(${APP_NAME} PUBLIC
    Qt6::Core
    Qt6::Gui
    Qt6::Qml
    Qt6::Quick
    Qt6::QuickControls2
    Qt6::Svg
    KF6::I18n
    KF6::CoreAddons
    KF6::ConfigCore
    KF6::ConfigGui
)
target_include_directories(${APP_NAME} PUBLIC ${CMAKE_BINARY_DIR})

if(NOT BUILD_WITH_MOCKS)
    include(${CMAKE_CURRENT_SOURCE_DIR}/realqmlmodules.cmake)
endif()

if(ANDROID)
    kirigami_package_breeze_icons(ICONS
        list-add
        help-about
        application-exit
        applications-graphics
    )
else()
    target_link_libraries(${APP_NAME} PUBLIC Qt::Widgets)
endif()

kconfig_add_kcfg_files(${APP_NAME} GENERATE_MOC frontendsexampleconfig.kcfgc)

# Target: main executable
add_executable(frontendsexample main.cpp)

if(BUILD_WITH_MOCKS)
    # copy mock_imports in binary folder
    file(COPY ${CMAKE_CURRENT_SOURCE_DIR}/mock_imports
        DESTINATION ${CMAKE_BINARY_DIR}/bin)
    target_compile_definitions(frontendsexample PRIVATE BUILD_WITH_MOCKS)
endif()

target_link_libraries(frontendsexample PUBLIC ${APP_NAME} ${APP_NAME}plugin)
install(TARGETS frontendsexample ${KDE_INSTALL_TARGETS_DEFAULT_ARGS})