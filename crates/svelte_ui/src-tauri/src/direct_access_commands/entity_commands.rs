use crate::AppContext;
use common::direct_access::entity::EntityRelationshipField;
use common::types::EntityId;
use direct_access::{entity_controller, CreateEntityDto, EntityDto, EntityRelationshipDto};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn create_entity(
    handle: tauri::AppHandle,
    dto: CreateEntityDto,
) -> Result<EntityDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    entity_controller::create(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error creating entity: {:?}", e))
}

#[tauri::command]
pub async fn create_entity_multi(
    handle: tauri::AppHandle,
    dtos: Vec<CreateEntityDto>,
) -> Result<Vec<EntityDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    entity_controller::create_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

#[tauri::command]
pub async fn get_entity(
    handle: tauri::AppHandle,
    id: EntityId,
) -> Result<Option<EntityDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    entity_controller::get(&app_context.db_context, &id)
        .map_err(|e| format!("Error getting entity: {:?}", e))
}

#[tauri::command]
pub async fn get_entity_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<Vec<Option<EntityDto>>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    entity_controller::get_multi(&app_context.db_context, ids.as_slice())
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

#[tauri::command]
pub async fn update_entity(handle: tauri::AppHandle, dto: EntityDto) -> Result<EntityDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    entity_controller::update(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error updating entity: {:?}", e))
}

#[tauri::command]
pub async fn update_entity_multi(
    handle: tauri::AppHandle,
    dtos: Vec<EntityDto>,
) -> Result<Vec<EntityDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    entity_controller::update_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

#[tauri::command]
pub async fn remove_entity(handle: tauri::AppHandle, id: EntityId) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    entity_controller::remove(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &id,
    )
    .map_err(|e| format!("Error deleting entity: {:?}", e))
}

#[tauri::command]
pub async fn remove_entity_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    entity_controller::remove_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        ids.as_slice(),
    )
    .map_err(|e| format!("Error deleting entities: {:?}", e))
}

#[tauri::command]
pub async fn get_entity_relationship(
    handle: tauri::AppHandle,
    id: EntityId,
    field: EntityRelationshipField,
) -> Result<Vec<EntityId>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    entity_controller::get_relationship(&app_context.db_context, &id, &field)
        .map_err(|e| format!("Error getting entity relationship: {:?}", e))
}

#[tauri::command]
pub async fn set_entity_relationship(
    handle: tauri::AppHandle,
    dto: EntityRelationshipDto,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    entity_controller::set_relationship(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error setting entity relationship: {:?}", e))
}
