use crate::AppContext;
use common::direct_access::dto::DtoRelationshipField;
use common::types::EntityId;
use direct_access::{dto_controller, CreateDtoDto, DtoDto, DtoRelationshipDto};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn create_dto(handle: tauri::AppHandle, dto: CreateDtoDto) -> Result<DtoDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_controller::create(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error creating dto: {:?}", e))
}

#[tauri::command]
pub async fn create_dto_multi(
    handle: tauri::AppHandle,
    dtos: Vec<CreateDtoDto>,
) -> Result<Vec<DtoDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_controller::create_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

#[tauri::command]
pub async fn get_dto(handle: tauri::AppHandle, id: EntityId) -> Result<Option<DtoDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    dto_controller::get(&app_context.db_context, &id)
        .map_err(|e| format!("Error getting dto: {:?}", e))
}

#[tauri::command]
pub async fn get_dto_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<Vec<Option<DtoDto>>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    dto_controller::get_multi(&app_context.db_context, ids.as_slice())
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

#[tauri::command]
pub async fn update_dto(handle: tauri::AppHandle, dto: DtoDto) -> Result<DtoDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_controller::update(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error updating dto: {:?}", e))
}

#[tauri::command]
pub async fn update_dto_multi(
    handle: tauri::AppHandle,
    dtos: Vec<DtoDto>,
) -> Result<Vec<DtoDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_controller::update_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

#[tauri::command]
pub async fn remove_dto(handle: tauri::AppHandle, id: EntityId) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_controller::remove(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &id,
    )
    .map_err(|e| format!("Error deleting dto: {:?}", e))
}

#[tauri::command]
pub async fn remove_dto_multi(handle: tauri::AppHandle, ids: Vec<EntityId>) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_controller::remove_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        ids.as_slice(),
    )
    .map_err(|e| format!("Error deleting entities: {:?}", e))
}

#[tauri::command]
pub async fn get_dto_relationship(
    handle: tauri::AppHandle,
    id: EntityId,
    field: DtoRelationshipField,
) -> Result<Vec<EntityId>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    dto_controller::get_relationship(&app_context.db_context, &id, &field)
        .map_err(|e| format!("Error getting dto relationship: {:?}", e))
}

#[tauri::command]
pub async fn set_dto_relationship(
    handle: tauri::AppHandle,
    dto: DtoRelationshipDto,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    dto_controller::set_relationship(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error setting dto relationship: {:?}", e))
}
