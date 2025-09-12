use crate::AppContext;
use common::direct_access::field::FieldRelationshipField;
use common::types::EntityId;
use direct_access::{field_controller, CreateFieldDto, FieldDto, FieldRelationshipDto};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn create_field(
    handle: tauri::AppHandle,
    dto: CreateFieldDto,
) -> Result<FieldDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    field_controller::create(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error creating field: {:?}", e))
}

#[tauri::command]
pub async fn create_field_multi(
    handle: tauri::AppHandle,
    dtos: Vec<CreateFieldDto>,
) -> Result<Vec<FieldDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    field_controller::create_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

#[tauri::command]
pub async fn get_field(handle: tauri::AppHandle, id: EntityId) -> Result<Option<FieldDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    field_controller::get(&app_context.db_context, &id)
        .map_err(|e| format!("Error getting field: {:?}", e))
}

#[tauri::command]
pub async fn get_field_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<Vec<Option<FieldDto>>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    field_controller::get_multi(&app_context.db_context, ids.as_slice())
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

#[tauri::command]
pub async fn update_field(handle: tauri::AppHandle, dto: FieldDto) -> Result<FieldDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    field_controller::update(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error updating field: {:?}", e))
}

#[tauri::command]
pub async fn update_field_multi(
    handle: tauri::AppHandle,
    dtos: Vec<FieldDto>,
) -> Result<Vec<FieldDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    field_controller::update_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

#[tauri::command]
pub async fn remove_field(handle: tauri::AppHandle, id: EntityId) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    field_controller::remove(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &id,
    )
    .map_err(|e| format!("Error deleting field: {:?}", e))
}

#[tauri::command]
pub async fn remove_field_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    field_controller::remove_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        ids.as_slice(),
    )
    .map_err(|e| format!("Error deleting entities: {:?}", e))
}

#[tauri::command]
pub async fn get_field_relationship(
    handle: tauri::AppHandle,
    id: EntityId,
    field: FieldRelationshipField,
) -> Result<Vec<EntityId>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    field_controller::get_relationship(&app_context.db_context, &id, &field)
        .map_err(|e| format!("Error getting field relationship: {:?}", e))
}

#[tauri::command]
pub async fn set_field_relationship(
    handle: tauri::AppHandle,
    dto: FieldRelationshipDto,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    field_controller::set_relationship(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error setting field relationship: {:?}", e))
}
