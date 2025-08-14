use crate::AppContext;
use rust_file_generation::{
    rust_file_generation_controller, GenerateRustCodeDto, GenerateRustCodeReturnDto,
    GenerateRustFilesDto, GenerateRustFilesReturnDto, ListRustFilesDto,
};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn list_rust_files(
    handle: tauri::AppHandle,
    dto: ListRustFilesDto,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    rust_file_generation_controller::list_rust_files(
        &app_context.db_context,
        &app_context.event_hub,
        &dto,
    )
    .map_err(|e| format!("Error while loading manifest: {:?}", e))?;

    // clear undo/redo stacks
    app_context.undo_redo_manager.lock().await.clear();
    Ok(())
}

#[tauri::command]
pub async fn generate_rust_code(
    handle: tauri::AppHandle,
    dto: GenerateRustCodeDto,
) -> Result<GenerateRustCodeReturnDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let return_dto =
        rust_file_generation_controller::generate_rust_code(&app_context.db_context, &dto)
            .map_err(|e| format!("Error while generating Rust code: {:?}", e))?;

    Ok(return_dto)
}

#[tauri::command]
pub async fn generate_rust_files(
    handle: tauri::AppHandle,
    dto: GenerateRustFilesDto,
) -> Result<String, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut long_operation_manager = app_context.long_operation_manager.lock().await;
    let operation_id = rust_file_generation_controller::generate_rust_files(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *long_operation_manager,
        &dto,
    )
    .map_err(|e| format!("Error while generating Rust files: {:?}", e))?;

    // clear undo/redo stacks
    app_context.undo_redo_manager.lock().await.clear();
    Ok(operation_id)
}

#[tauri::command]
pub async fn get_generate_rust_files_result(
    handle: tauri::AppHandle,
    operation_id: String,
) -> Result<Option<GenerateRustFilesReturnDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let long_operation_manager = app_context.long_operation_manager.lock().await;

    rust_file_generation_controller::get_generate_rust_files_result(
        &*long_operation_manager,
        &operation_id,
    )
    .map_err(|e| {
        format!(
            "Error while retrieving Rust files generation result: {:?}",
            e
        )
    })
}
