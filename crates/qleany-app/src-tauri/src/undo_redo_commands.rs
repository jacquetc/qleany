use tauri::State;
use tauri::async_runtime::Mutex;
use crate::AppContext;

/// Undoes the most recent command.
#[tauri::command]
pub async fn undo(app_context: State<'_, Mutex<AppContext>>) -> Result<(), String> {
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    
    undo_redo_manager.undo().map_err(|e| e.to_string())
}

/// Redoes the most recently undone command.
#[tauri::command]
pub async fn redo(app_context: State<'_, Mutex<AppContext>>) -> Result<(), String> {
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    
    undo_redo_manager.redo().map_err(|e| e.to_string())
}

/// Checks if there are commands that can be undone.
#[tauri::command]
pub async fn can_undo(app_context: State<'_, Mutex<AppContext>>) -> Result<bool, String> {
    let app_context = app_context.lock().await;
    let undo_redo_manager = app_context.undo_redo_manager.lock().await;
    
    Ok(undo_redo_manager.can_undo())
}

/// Checks if there are commands that can be redone.
#[tauri::command]
pub async fn can_redo(app_context: State<'_, Mutex<AppContext>>) -> Result<bool, String> {
    let app_context = app_context.lock().await;
    let undo_redo_manager = app_context.undo_redo_manager.lock().await;
    
    Ok(undo_redo_manager.can_redo())
}

/// Begins a composite command group.
#[tauri::command]
pub async fn begin_composite(app_context: State<'_, Mutex<AppContext>>) -> Result<(), String> {
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    
    undo_redo_manager.begin_composite();
    Ok(())
}

/// Ends the current composite command group.
#[tauri::command]
pub async fn end_composite(app_context: State<'_, Mutex<AppContext>>) -> Result<(), String> {
    let app_context = app_context.lock().await;
    let mut undo_redo_manager = app_context.undo_redo_manager.lock().await;
    
    undo_redo_manager.end_composite();
    Ok(())
}