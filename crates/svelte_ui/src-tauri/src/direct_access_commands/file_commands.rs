use crate::AppContext;
use common::types::EntityId;
use direct_access::{file_controller, CreateFileDto, FileDto};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn create_file(handle: tauri::AppHandle, dto: CreateFileDto) -> Result<FileDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    file_controller::create(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error creating file: {:?}", e))
}

#[tauri::command]
pub async fn create_file_multi(
    handle: tauri::AppHandle,
    dtos: Vec<CreateFileDto>,
) -> Result<Vec<FileDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    file_controller::create_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

#[tauri::command]
pub async fn get_file(handle: tauri::AppHandle, id: EntityId) -> Result<Option<FileDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    file_controller::get(&app_context.db_context, &id)
        .map_err(|e| format!("Error getting file: {:?}", e))
}

#[tauri::command]
pub async fn get_file_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<Vec<Option<FileDto>>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    file_controller::get_multi(&app_context.db_context, ids.as_slice())
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

#[tauri::command]
pub async fn update_file(handle: tauri::AppHandle, dto: FileDto) -> Result<FileDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    file_controller::update(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error updating file: {:?}", e))
}

#[tauri::command]
pub async fn update_file_multi(
    handle: tauri::AppHandle,
    dtos: Vec<FileDto>,
) -> Result<Vec<FileDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    file_controller::update_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

#[tauri::command]
pub async fn remove_file(handle: tauri::AppHandle, id: EntityId) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    file_controller::remove(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &id,
    )
    .map_err(|e| format!("Error deleting file: {:?}", e))
}

#[tauri::command]
pub async fn remove_file_multi(handle: tauri::AppHandle, ids: Vec<EntityId>) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    file_controller::remove_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        ids.as_slice(),
    )
    .map_err(|e| format!("Error deleting entities: {:?}", e))
}
