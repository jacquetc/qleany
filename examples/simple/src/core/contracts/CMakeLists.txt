# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core Sql REQUIRED)
# find_package(qleany CONFIG REQUIRED)

set(LIBRARY_NAME simple-example-contracts)

include(repository_interfaces.cmake)

qt_add_library(${LIBRARY_NAME} STATIC ${REPOSITORY_LIST})

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
    BASE_NAME simple_example_contracts
)

set_target_properties(${LIBRARY_NAME} PROPERTIES
                    POSITION_INDEPENDENT_CODE ${BUILD_SHARED_LIBS}
                    )

target_include_directories(${LIBRARY_NAME} PUBLIC 
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_BINARY_DIR}
)

target_link_libraries(${LIBRARY_NAME} PRIVATE simple-example-entities)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core Qt6::Sql
)
target_link_libraries(${LIBRARY_NAME} PRIVATE qleany)