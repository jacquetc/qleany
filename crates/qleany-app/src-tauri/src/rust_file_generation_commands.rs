use crate::AppContext;
use rust_file_generation::{
    rust_file_generation_controller, GenerateRustFilesDto, ListRustFilesDto,
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
