
set(APP_NAME simple-example-desktop-application)

# Create an executable target for the example project
add_executable(${APP_NAME}
    main.cpp
    mainwindow.h mainwindow.cpp mainwindow.ui
    # add other source files for the example
)

target_link_libraries(${APP_NAME} PUBLIC Qt6::Core Qt6::Widgets Qt6::Concurrent Qt6::Sql)
# Link the example against the Qleany library
target_link_libraries(${APP_NAME} PUBLIC Qleany)
target_link_libraries(${APP_NAME} PUBLIC simple-example-domain)
target_link_libraries(${APP_NAME} PUBLIC simple-example-persistence)
target_link_libraries(${APP_NAME} PUBLIC simple-example-controller)
target_link_libraries(${APP_NAME} PUBLIC simple-example-presenter)
target_link_libraries(${APP_NAME} PRIVATE QCoro::Core)
