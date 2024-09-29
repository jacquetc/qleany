# SPDX-FileCopyrightText: %{CURRENT_YEAR} %{AUTHOR} <%{EMAIL}>
# SPDX-License-Identifier: BSD-3-Clause
# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

cmake_minimum_required(VERSION 3.16)

project(frontendsexample VERSION 0.1)

# Compiler settings
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(BUILD_SHARED_LIBS "Build shared libraries" OFF)
option(BUILD_QT_QUICK "Build the QtQuick front end" ON)

option(BUILD_QT_WIDGETS "Build the QtWidgets front end" OFF)

option(BUILD_KF6_KIRIGAMI "Build the KF6Kirigami front end" OFF)



if(NOT BUILD_QT_QUICK AND NOT BUILD_QT_WIDGETS AND NOT BUILD_KF6_KIRIGAMI AND NOT BUILD_KF6_WIDGETS)
    message(FATAL_ERROR "At least one front end must be enabled")
endif()

if(BUILD_KF6_KIRIGAMI OR BUILD_KF6_WIDGETS)
    set(BUILD_KF6 ON)
endif()

if(BUILD_KF6)
    set(QT6_MIN_VERSION 6.5.0)
    set(KF6_MIN_VERSION 6.0.0)

    find_package(ECM ${KF6_MIN_VERSION} REQUIRED NO_MODULE)
    set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${ECM_MODULE_PATH})

    include(FeatureSummary)
    include(KDEInstallDirs)
    include(KDECMakeSettings)
    include(KDECompilerSettings NO_POLICY_SCOPE)
    include(ECMSetupVersion)
    include(ECMFindQmlModule)
    include(ECMInstallIcons)


    # enable -fexceptions
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fexceptions")

    if(NOT ANDROID)
        include(KDEGitCommitHooks)
        include(KDEClangFormat)
    endif()

if(BUILD_KF6_KIRIGAMI)
    ecm_setup_version(${PROJECT_VERSION}
        VARIABLE_PREFIX FRONTENDSEXAMPLE
        VERSION_HEADER "${CMAKE_CURRENT_BINARY_DIR}/src/gui/kf6_kirigami_application//version-frontendsexample.h"
    )
endif()


endif()

if(BUILD_QT_QUICK OR BUILD_QT_WIDGETS)
    set(QT6_MIN_VERSION 6.5.0)
endif()



# Add QCoro
find_package(QCoro6 REQUIRED COMPONENTS Core)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

find_package(Qt6 ${QT6_MIN_VERSION} REQUIRED COMPONENTS Core Gui Svg Sql Concurrent)

if(BUILD_KF6_KIRIGAMI OR BUILD_QT_QUICK)
    find_package(Qt6 ${QT6_MIN_VERSION} REQUIRED COMPONENTS Qml QuickControls2)
    find_package(QCoro6 REQUIRED COMPONENTS Qml)

    set(QT_EXTRA_COMPONENTS)

    if(NOT ANDROID)
        list(APPEND QT_EXTRA_COMPONENTS Widgets)
    endif()

    find_package(Qt6 ${QT6_MIN_VERSION} REQUIRED COMPONENTS Qml QuickControls2 ${QT_EXTRA_COMPONENTS})
endif()

if(BUILD_KF6_KIRIGAMI)
    find_package(KF6 ${KF6_MIN_VERSION} REQUIRED COMPONENTS Kirigami CoreAddons Config I18n)

    if(ANDROID)
        configure_file(${CMAKE_CURRENT_SOURCE_DIR}/android/version.gradle.in ${CMAKE_BINARY_DIR}/version.gradle)
    endif()

    qt_policy(SET QTP0001 NEW)

    ecm_find_qmlmodule(org.kde.kirigamiaddons.formcard 1.0)
endif()


# handles the common code
add_subdirectory(src/core/common/)

# handles the entities
add_subdirectory(src/core/entities/)

# stores the interfaces
add_subdirectory(src/core/contracts)

# handles the data access from inside the system
add_subdirectory(src/core/persistence)

# handles the data access from outside the system
add_subdirectory(src/core/contracts.dto)

# handles the CQRS commands, queries and validators
add_subdirectory(src/core/contracts.cqrs)

# handles the use cases
add_subdirectory(src/core/application)

# handles the interaction between the system's inner layers (use cases, DTOs) and the external world
add_subdirectory(src/core/controller/)

# handles the Qt models
add_subdirectory(src/gui/presenter)

if(BUILD_QT_QUICK)
    # handles the QtQuick front end
    add_subdirectory(src/gui/qt_quick_application/)
endif()

if(BUILD_QT_WIDGETS)
    # handles the QtWidgets front end
    add_subdirectory(src/gui/qt_widgets_application/)
endif()

if(BUILD_KF6_KIRIGAMI)
    # handles the Kirigami front end
    add_subdirectory(src/gui/kf6_kirigami_application/)
endif()





if(BUILD_KF6)
    install(FILES org.kde.frontendsexample.desktop DESTINATION ${KDE_INSTALL_APPDIR})
    install(FILES org.kde.frontendsexample.metainfo.xml DESTINATION ${KDE_INSTALL_METAINFODIR})

    ki18n_install(po)

    feature_summary(WHAT ALL INCLUDE_QUIET_PACKAGES FATAL_ON_MISSING_REQUIRED_PACKAGES)

    if(NOT ANDROID)
        file(GLOB_RECURSE ALL_CLANG_FORMAT_SOURCE_FILES src/*.cpp src/*.h)
        kde_clang_format(${ALL_CLANG_FORMAT_SOURCE_FILES})
        kde_configure_git_pre_commit_hook(CHECKS CLANG_FORMAT)
    endif()
endif()
    