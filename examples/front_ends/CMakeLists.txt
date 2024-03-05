cmake_minimum_required(VERSION 3.21)

project(FrontEndsExample LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)

# Compiler settings
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(BUILD_SHARED_LIBS "Build shared libraries" OFF)

# Add Qt components
find_package(Qt6 COMPONENTS Core Sql Concurrent REQUIRED)
qt_standard_project_setup()

# Add QCoro
find_package(QCoro6 REQUIRED COMPONENTS Core)
# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

# handles the entities
add_subdirectory(src/core/entities)
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
# handles the interaction between the system's inner layers (use cases, DTOs) and the external world 
add_subdirectory(src/core/interactor/) 
# handles the Qt models
add_subdirectory(src/gui/presenter) 

# handles the QtQuick front end
#add_subdirectory(src/gui/qt_quick_application/)

# handles the QtWidgets front end
add_subdirectory(src/gui/qt_widgets_application/)

# handles the Kirigami front end
#add_subdirectory(src/gui/kf6_kirigami_application/)

# handles the KF6Widgets front end
#add_subdirectory(src/gui/kf6_widgets_application/)
