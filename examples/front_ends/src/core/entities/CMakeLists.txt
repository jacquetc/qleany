# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core REQUIRED)

set(LIBRARY_NAME front-ends-example-entities)

include(entities.cmake)

set(SRCS
    entities.h
    entities_registration.h
    entity_base.h
    entity_base.cpp
    entity_schema.h
)

qt_add_library(${LIBRARY_NAME} STATIC ${SRCS} ${ENTITY_LIST})

set_target_properties(${LIBRARY_NAME} PROPERTIES
                      POSITION_INDEPENDENT_CODE ${BUILD_SHARED_LIBS}
                      )

set_target_properties(${LIBRARY_NAME} PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(${LIBRARY_NAME} PUBLIC 
    ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(${LIBRARY_NAME} PRIVATE Qt6::Core )