# SPDX-FileCopyrightText: %{CURRENT_YEAR} %{AUTHOR} <%{EMAIL}>
# SPDX-License-Identifier: BSD-3-Clause This file was generated automatically by
# Qleany's generator, edit at your own risk! If you do, be careful to not
# overwrite it when you run the generator again.

cmake_minimum_required(VERSION 3.16)

project(frontendsexample VERSION 0.1)

# Compiler settings
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(BUILD_SHARED_LIBS "Build shared libraries" OFF)
option(BUILD_QT_QUICK "Build the QtQuick front end" ON)

option(BUILD_QT_WIDGETS "Build the QtWidgets front end" ON)

if(NOT BUILD_QT_QUICK
   AND NOT BUILD_QT_WIDGETS
   AND NOT BUILD_KF6_KIRIGAMI
   AND NOT BUILD_KF6_WIDGETS)
  message(FATAL_ERROR "At least one front end must be enabled")
endif()

if(BUILD_QT_QUICK OR BUILD_QT_WIDGETS)
  set(QT6_MIN_VERSION 6.5.0)
endif()

# Add QCoro
find_package(QCoro6 REQUIRED COMPONENTS Core)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

find_package(Qt6 ${QT6_MIN_VERSION} REQUIRED COMPONENTS Core Gui Svg Sql
                                                        Concurrent)
if(BUILD_KF6_KIRIGAMI OR BUILD_QT_QUICK)
  find_package(Qt6 ${QT6_MIN_VERSION} REQUIRED COMPONENTS Qml QuickControls2)

  set(QT_EXTRA_COMPONENTS)

  if(NOT ANDROID)
    list(APPEND QT_EXTRA_COMPONENTS Widgets)
  endif()

  find_package(Qt6 ${QT6_MIN_VERSION} REQUIRED
               COMPONENTS Qml QuickControls2 ${QT_EXTRA_COMPONENTS})
endif()

# handles the entities
add_subdirectory(src/core/entities/)

# stores the interfaces
add_subdirectory(src/core/contracts)

# handles the data access from inside the system
add_subdirectory(src/core/persistence/)

# handles the data access from outside the system
add_subdirectory(src/core/contracts.dto)

# handles the CQRS commands, queries and validators
add_subdirectory(src/core/contracts.cqrs)

# handles the use cases
add_subdirectory(src/core/application)

# handles the interaction between the system's inner layers (use cases, DTOs)
# and the external world
add_subdirectory(src/core/interactor/)

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
