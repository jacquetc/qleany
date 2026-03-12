find_package(Qt6 REQUIRED COMPONENTS Core Sql Test)
find_package(QCoro6 REQUIRED COMPONENTS Core Test)

# Convenience macro: define a functional test that links the tested project
macro(add_functional_test TEST_NAME TEST_SOURCE)
    add_executable(${TEST_NAME} ${TEST_SOURCE})

    target_compile_definitions(${TEST_NAME} PRIVATE QT_NO_FOREACH)

    target_link_libraries(${TEST_NAME} PRIVATE
        Qt6::Core
        Qt6::Sql
        Qt6::Test
        QCoro::Core
        QCoro::Test
        full-cpp-qt-app-common
        full-cpp-qt-app-direct-access
    )

    qcoro_enable_coroutines(${TEST_NAME})

    # Include paths for the tested project's headers
    target_include_directories(${TEST_NAME} PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/common
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/common/entities
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/common/database
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/common/direct_access
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/common/undo_redo
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/common/unit_of_work
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/common/long_operation
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/common/features
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/direct_access
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/direct_access/root
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/direct_access/tag
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/direct_access/task
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/direct_access/workspace
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/direct_access/project
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/direct_access/project_settings
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/direct_access/comment
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/direct_access/category
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src/direct_access/team_member
        ${CMAKE_CURRENT_SOURCE_DIR}/../tested_project/src
    )

    add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})
endmacro()

add_functional_test(functional_tst_root_controller tst_root_controller.cpp)
add_functional_test(functional_tst_tag_controller tst_tag_controller.cpp)
add_functional_test(functional_tst_task_controller tst_task_controller.cpp)
add_functional_test(functional_tst_undo_redo tst_undo_redo.cpp)