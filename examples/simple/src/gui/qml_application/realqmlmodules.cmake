add_subdirectory(real_imports)

target_link_libraries(${APP_NAME} PRIVATE
controllersplugin
modelplugin
)