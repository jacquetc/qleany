add_subdirectory(real_imports)

target_link_libraries(${APP_NAME} PRIVATE
    simple-example-qml-controllersplugin
    simple-example-qml-modelsplugin
    simple-example-qml-singlesplugin
)
