use crate::AppContext;
use direct_access::{entity_controller, CreateEntityDto, EntityDto};
use std::ops::{Deref, DerefMut};
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

// #[tauri::command]
// pub async fn get_entities(
//     handle: tauri::AppHandle,
// ) -> Result<Vec<EntityDto>, String> {
//     let app_context = handle.state::<Mutex<AppContext>>();
//     let app_context = app_context.lock().await;
//     entity_controller::get_multi(&app_context.db_context, &app_context.event_hub)
//         .map_err(|e| format!("Error getting entities: {:?}", e))
// }
