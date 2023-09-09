
# Create an executable target for the example project
add_executable(QleanySimpleExample
    main.cpp
    mainwindow.h mainwindow.cpp mainwindow.ui
    # add other source files for the example
)

target_link_libraries(QleanySimpleExample PUBLIC Qt6::Core Qt6::Widgets Qt6::Concurrent Qt6::Sql)
# Link the example against the Qleany library
target_link_libraries(QleanySimpleExample PUBLIC Qleany)
target_link_libraries(QleanySimpleExample PUBLIC simple-example-domain)
target_link_libraries(QleanySimpleExample PUBLIC simple-example-persistence)
target_link_libraries(QleanySimpleExample PUBLIC simple-example-controller)
target_link_libraries(QleanySimpleExample PUBLIC simple-example-presenter)
target_link_libraries(QleanySimpleExample PRIVATE QCoro::Core)
