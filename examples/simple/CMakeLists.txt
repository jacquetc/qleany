cmake_minimum_required(VERSION 3.21)

project(SimpleExample LANGUAGES CXX)

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