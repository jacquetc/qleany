use crate::AppContext;
use handling_manifest::{handling_manifest_controller, LoadDto};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn load_manifest(handle: tauri::AppHandle, dto: LoadDto) -> Result<(), String> {
    let app_context = handle.state::<Mutex<AppContext>>();
    let app_context = app_context.lock().await;
    handling_manifest_controller::load(&app_context.db_context, &app_context.event_hub, &dto)
        .map_err(|e| format!("Error while loading manifest: {:?}", e))
}
