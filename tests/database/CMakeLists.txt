cmake_minimum_required(VERSION 3.18)

#-----------------------------------------------------------------------------------

set(TEST_NAME "tst_database_table")

project(${TEST_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core Sql REQUIRED)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
add_executable(${TEST_NAME} ${TEST_NAME}.cpp ${RESOURCES}
    dummy_database_context.h
    dummy_entity.h
    dummy_basic_entity.h
    entities.h
)
add_test(${TEST_NAME} ${TEST_NAME})

target_link_libraries(${TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql )

target_link_libraries(${TEST_NAME} PUBLIC Qleany)

#-----------------------------------------------------------------------------------

set(TEST_NAME "tst_foreign_entity")

project(${TEST_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Test Core Sql CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core Sql REQUIRED)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
add_executable(${TEST_NAME} ${TEST_NAME}.cpp ${RESOURCES}
    dummy_database_context.h
    dummy_entity.h
    dummy_entity_with_foreign.h
    dummy_other_entity.h
    otm_ordered_wrapper.h
    entities.h
    qleany.yaml
)
add_test(${TEST_NAME} ${TEST_NAME})

target_link_libraries(${TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql )

target_link_libraries(${TEST_NAME} PUBLIC Qleany)


#-----------------------------------------------------------------------------------

set(TEST_NAME "tst_database_tools")

project(${TEST_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Test Core Sql CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core Sql REQUIRED)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
add_executable(${TEST_NAME} ${TEST_NAME}.cpp ${RESOURCES}
    dummy_entity.h
    dummy_database_context.h
    dummy_basic_entity.h
    entities.h
)
add_test(${TEST_NAME} ${TEST_NAME})

target_link_libraries(${TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql )

target_link_libraries(${TEST_NAME} PUBLIC Qleany)
