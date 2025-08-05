use tauri::State;
use tauri::async_runtime::Mutex;
use crate::AppContext;
use common::long_operation::{OperationProgress as CommonOperationProgress, OperationStatus as CommonOperationStatus};
use serde::{Deserialize, Serialize};

// Serializable version of OperationStatus for Tauri commands
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OperationStatus {
    Running,
    Completed,
    Cancelled,
    Failed(String),
}

impl From<CommonOperationStatus> for OperationStatus {
    fn from(status: CommonOperationStatus) -> Self {
        match status {
            CommonOperationStatus::Running => OperationStatus::Running,
            CommonOperationStatus::Completed => OperationStatus::Completed,
            CommonOperationStatus::Cancelled => OperationStatus::Cancelled,
            CommonOperationStatus::Failed(msg) => OperationStatus::Failed(msg),
        }
    }
}

// Serializable version of OperationProgress for Tauri commands
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OperationProgress {
    pub percentage: f32,
    pub message: Option<String>,
}

impl From<CommonOperationProgress> for OperationProgress {
    fn from(progress: CommonOperationProgress) -> Self {
        OperationProgress {
            percentage: progress.percentage,
            message: progress.message,
        }
    }
}

/// Gets the status of a long operation.
#[tauri::command]
pub async fn get_operation_status(
    app_context: State<'_, Mutex<AppContext>>,
    operation_id: String,
) -> Result<Option<OperationStatus>, String> {
    let app_context = app_context.lock().await;
    let long_operation_manager = app_context.long_operation_manager.lock().await;
    
    Ok(long_operation_manager.get_operation_status(&operation_id)
        .map(|status| status.into()))
}

/// Gets the progress of a long operation.
#[tauri::command]
pub async fn get_operation_progress(
    app_context: State<'_, Mutex<AppContext>>,
    operation_id: String,
) -> Result<Option<OperationProgress>, String> {
    let app_context = app_context.lock().await;
    let long_operation_manager = app_context.long_operation_manager.lock().await;
    
    Ok(long_operation_manager.get_operation_progress(&operation_id)
        .map(|progress| progress.into()))
}

/// Cancels a long operation.
#[tauri::command]
pub async fn cancel_operation(
    app_context: State<'_, Mutex<AppContext>>,
    operation_id: String,
) -> Result<bool, String> {
    let app_context = app_context.lock().await;
    let long_operation_manager = app_context.long_operation_manager.lock().await;
    
    Ok(long_operation_manager.cancel_operation(&operation_id))
}

/// Checks if an operation is finished.
#[tauri::command]
pub async fn is_operation_finished(
    app_context: State<'_, Mutex<AppContext>>,
    operation_id: String,
) -> Result<Option<bool>, String> {
    let app_context = app_context.lock().await;
    let long_operation_manager = app_context.long_operation_manager.lock().await;
    
    Ok(long_operation_manager.is_operation_finished(&operation_id))
}

/// Cleans up finished operations.
#[tauri::command]
pub async fn cleanup_finished_operations(
    app_context: State<'_, Mutex<AppContext>>,
) -> Result<(), String> {
    let app_context = app_context.lock().await;
    let mut long_operation_manager = app_context.long_operation_manager.lock().await;
    
    long_operation_manager.cleanup_finished_operations();
    Ok(())
}

/// Lists all operation IDs.
#[tauri::command]
pub async fn list_operations(
    app_context: State<'_, Mutex<AppContext>>,
) -> Result<Vec<String>, String> {
    let app_context = app_context.lock().await;
    let long_operation_manager = app_context.long_operation_manager.lock().await;
    
    Ok(long_operation_manager.list_operations())
}

/// Gets a summary of all operations.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OperationSummary {
    pub id: String,
    pub status: OperationStatus,
    pub progress: OperationProgress,
}

#[tauri::command]
pub async fn get_operations_summary(
    app_context: State<'_, Mutex<AppContext>>,
) -> Result<Vec<OperationSummary>, String> {
    let app_context = app_context.lock().await;
    let long_operation_manager = app_context.long_operation_manager.lock().await;
    
    let summaries = long_operation_manager.get_operations_summary();
    let result = summaries
        .into_iter()
        .map(|(id, status, progress)| OperationSummary {
            id,
            status: status.into(),
            progress: progress.into(),
        })
        .collect();
    
    Ok(result)
}