use crate::AppContext;
use common::direct_access::use_case::UseCaseRelationshipField;
use common::types::EntityId;
use direct_access::{use_case_controller, CreateUseCaseDto, UseCaseDto, UseCaseRelationshipDto};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn create_use_case(
    handle: tauri::AppHandle,
    dto: CreateUseCaseDto,
) -> Result<UseCaseDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    use_case_controller::create(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error creating use_case: {:?}", e))
}

#[tauri::command]
pub async fn create_use_case_multi(
    handle: tauri::AppHandle,
    dtos: Vec<CreateUseCaseDto>,
) -> Result<Vec<UseCaseDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    use_case_controller::create_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

#[tauri::command]
pub async fn get_use_case(
    handle: tauri::AppHandle,
    id: EntityId,
) -> Result<Option<UseCaseDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    use_case_controller::get(&app_context.db_context, &id)
        .map_err(|e| format!("Error getting use_case: {:?}", e))
}

#[tauri::command]
pub async fn get_use_case_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<Vec<Option<UseCaseDto>>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    use_case_controller::get_multi(&app_context.db_context, ids.as_slice())
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

#[tauri::command]
pub async fn update_use_case(
    handle: tauri::AppHandle,
    dto: UseCaseDto,
) -> Result<UseCaseDto, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    use_case_controller::update(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error updating use_case: {:?}", e))
}

#[tauri::command]
pub async fn update_use_case_multi(
    handle: tauri::AppHandle,
    dtos: Vec<UseCaseDto>,
) -> Result<Vec<UseCaseDto>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    use_case_controller::update_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        dtos.as_slice(),
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

#[tauri::command]
pub async fn remove_use_case(handle: tauri::AppHandle, id: EntityId) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    use_case_controller::remove(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &id,
    )
    .map_err(|e| format!("Error deleting use_case: {:?}", e))
}

#[tauri::command]
pub async fn remove_use_case_multi(
    handle: tauri::AppHandle,
    ids: Vec<EntityId>,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    use_case_controller::remove_multi(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        ids.as_slice(),
    )
    .map_err(|e| format!("Error deleting entities: {:?}", e))
}

#[tauri::command]
pub async fn get_use_case_relationship(
    handle: tauri::AppHandle,
    id: EntityId,
    field: UseCaseRelationshipField,
) -> Result<Vec<EntityId>, String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    use_case_controller::get_relationship(&app_context.db_context, &id, &field)
        .map_err(|e| format!("Error getting use_case relationship: {:?}", e))
}

#[tauri::command]
pub async fn set_use_case_relationship(
    handle: tauri::AppHandle,
    dto: UseCaseRelationshipDto,
) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    use_case_controller::set_relationship(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        &dto,
    )
    .map_err(|e| format!("Error setting use_case relationship: {:?}", e))
}
