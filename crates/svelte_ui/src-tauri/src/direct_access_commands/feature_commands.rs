use crate::AppContext;
use common::direct_access::feature::FeatureRelationshipField;
use common::types::EntityId;
use direct_access::{feature_controller, CreateFeatureDto, FeatureDto, FeatureRelationshipDto};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn create_feature(
    handle: tauri::AppHandle,
    dto: CreateFeatureDto,
) -> Result<FeatureDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    feature_controller::create(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error creating feature: {:?}", e))
}

#[tauri::command]
pub async fn create_feature_multi(
    handle: tauri::AppHandle,
    dtos: Vec<CreateFeatureDto>,
) -> Result<Vec<FeatureDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    feature_controller::create_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

#[tauri::command]
pub async fn get_feature(
    handle: tauri::AppHandle,
    id: EntityId,
) -> Result<Option<FeatureDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    feature_controller::get(&app_context.db_context, &id)
        .map_err(|e| format!("Error getting feature: {:?}", e))
}

#[tauri::command]
pub async fn get_feature_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<Vec<Option<FeatureDto>>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    feature_controller::get_multi(&app_context.db_context, ids.as_slice())
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

#[tauri::command]
pub async fn update_feature(
    handle: tauri::AppHandle,
    dto: FeatureDto,
) -> Result<FeatureDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    feature_controller::update(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error updating feature: {:?}", e))
}

#[tauri::command]
pub async fn update_feature_multi(
    handle: tauri::AppHandle,
    dtos: Vec<FeatureDto>,
) -> Result<Vec<FeatureDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    feature_controller::update_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

#[tauri::command]
pub async fn remove_feature(handle: tauri::AppHandle, id: EntityId) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    feature_controller::remove(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &id,
    )
    .map_err(|e| format!("Error deleting feature: {:?}", e))
}

#[tauri::command]
pub async fn remove_feature_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    feature_controller::remove_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        ids.as_slice(),
    )
    .map_err(|e| format!("Error deleting entities: {:?}", e))
}

#[tauri::command]
pub async fn get_feature_relationship(
    handle: tauri::AppHandle,
    id: EntityId,
    field: FeatureRelationshipField,
) -> Result<Vec<EntityId>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    feature_controller::get_relationship(&app_context.db_context, &id, &field)
        .map_err(|e| format!("Error getting feature relationship: {:?}", e))
}

#[tauri::command]
pub async fn set_feature_relationship(
    handle: tauri::AppHandle,
    dto: FeatureRelationshipDto,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    feature_controller::set_relationship(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error setting feature relationship: {:?}", e))
}
