use crate::AppContext;
use common::direct_access::relationship::RelationshipRelationshipField;
use common::types::EntityId;
use direct_access::{
    relationship_controller, CreateRelationshipDto, RelationshipDto, RelationshipRelationshipDto,
};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn create_relationship(
    handle: tauri::AppHandle,
    dto: CreateRelationshipDto,
) -> Result<RelationshipDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    relationship_controller::create(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error creating relationship: {:?}", e))
}

#[tauri::command]
pub async fn create_relationship_multi(
    handle: tauri::AppHandle,
    dtos: Vec<CreateRelationshipDto>,
) -> Result<Vec<RelationshipDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    relationship_controller::create_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

#[tauri::command]
pub async fn get_relationship(
    handle: tauri::AppHandle,
    id: EntityId,
) -> Result<Option<RelationshipDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    relationship_controller::get(&app_context.db_context, &id)
        .map_err(|e| format!("Error getting relationship: {:?}", e))
}

#[tauri::command]
pub async fn get_relationship_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<Vec<Option<RelationshipDto>>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    relationship_controller::get_multi(&app_context.db_context, ids.as_slice())
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

#[tauri::command]
pub async fn update_relationship(
    handle: tauri::AppHandle,
    dto: RelationshipDto,
) -> Result<RelationshipDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    relationship_controller::update(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error updating relationship: {:?}", e))
}

#[tauri::command]
pub async fn update_relationship_multi(
    handle: tauri::AppHandle,
    dtos: Vec<RelationshipDto>,
) -> Result<Vec<RelationshipDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    relationship_controller::update_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

#[tauri::command]
pub async fn remove_relationship(handle: tauri::AppHandle, id: EntityId) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    relationship_controller::remove(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &id,
    )
    .map_err(|e| format!("Error deleting relationship: {:?}", e))
}

#[tauri::command]
pub async fn remove_relationship_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    relationship_controller::remove_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        ids.as_slice(),
    )
    .map_err(|e| format!("Error deleting entities: {:?}", e))
}

#[tauri::command]
pub async fn get_relationship_relationship(
    handle: tauri::AppHandle,
    id: EntityId,
    field: RelationshipRelationshipField,
) -> Result<Vec<EntityId>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    relationship_controller::get_relationship(&app_context.db_context, &id, &field)
        .map_err(|e| format!("Error getting relationship relationship: {:?}", e))
}

#[tauri::command]
pub async fn set_relationship_relationship(
    handle: tauri::AppHandle,
    dto: RelationshipRelationshipDto,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    relationship_controller::set_relationship(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error setting relationship relationship: {:?}", e))
}
