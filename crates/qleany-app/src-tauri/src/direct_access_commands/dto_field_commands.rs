use crate::AppContext;
use common::types::EntityId;
use direct_access::{dto_field_controller, CreateDtoFieldDto, DtoFieldDto};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn create_dto_field(
    handle: tauri::AppHandle,
    dto: CreateDtoFieldDto,
) -> Result<DtoFieldDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_field_controller::create(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error creating dto_field: {:?}", e))
}

#[tauri::command]
pub async fn create_dto_field_multi(
    handle: tauri::AppHandle,
    dtos: Vec<CreateDtoFieldDto>,
) -> Result<Vec<DtoFieldDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_field_controller::create_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

#[tauri::command]
pub async fn get_dto_field(
    handle: tauri::AppHandle,
    id: EntityId,
) -> Result<Option<DtoFieldDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    dto_field_controller::get(&app_context.db_context, &id)
        .map_err(|e| format!("Error getting dto_field: {:?}", e))
}

#[tauri::command]
pub async fn get_dto_field_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<Vec<Option<DtoFieldDto>>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    dto_field_controller::get_multi(&app_context.db_context, ids.as_slice())
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

#[tauri::command]
pub async fn update_dto_field(
    handle: tauri::AppHandle,
    dto: DtoFieldDto,
) -> Result<DtoFieldDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_field_controller::update(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error updating dto_field: {:?}", e))
}

#[tauri::command]
pub async fn update_dto_field_multi(
    handle: tauri::AppHandle,
    dtos: Vec<DtoFieldDto>,
) -> Result<Vec<DtoFieldDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_field_controller::update_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

#[tauri::command]
pub async fn remove_dto_field(handle: tauri::AppHandle, id: EntityId) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_field_controller::remove(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &id,
    )
    .map_err(|e| format!("Error deleting dto_field: {:?}", e))
}

#[tauri::command]
pub async fn remove_dto_field_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_field_controller::remove_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        ids.as_slice(),
    )
    .map_err(|e| format!("Error deleting entities: {:?}", e))
}
