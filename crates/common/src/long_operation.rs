//! This module provides a framework for managing long-running operations with the ability to track
//! status, progress, and enable cancellation. It includes the infrastructure for defining, executing,
//! and monitoring such operations. For undoable operations, it is recommended to use the Undo/Redo framework.
//!
//! # Components:
//!
//! - **OperationStatus**: Enum representing the state of an operation.
//! - **OperationProgress**: Struct holding details about the progress of an operation.
//! - **LongOperation**: Trait that must be implemented by any long-running operation.
//! - **LongOperationManager**: Manager that orchestrates the execution, tracking, and cleanup of multiple operations.
//!
//! # Usage:
//!
//! 1. Implement the `LongOperation` trait for your task.
//! 2. Use `LongOperationManager` to start, track, and manage your operations.
//! 3. Access methods like:
//!     - `start_operation` to start new operations.
//!     - `get_operation_status`, `get_operation_progress` to query operation details.
//!     - `cancel_operation` to cancel operations.
//!     - `cleanup_finished_operations` to remove completed or cancelled operations.
//!
//! # Example:
//!
//! ```rust
//! // Define your long-running operation
//! use std::sync::Arc;
//! use std::sync::atomic::{AtomicBool, Ordering};
//! use std::thread;
//! use std::time::Duration;
//! use common::long_operation::{LongOperation, LongOperationManager, OperationProgress};
//!
//! pub struct MyOperation {
//!     pub total_steps: usize,
//! }
//!
//! impl LongOperation for MyOperation {
//!     fn execute(
//!         &self,
//!         progress_callback: Box<dyn Fn(OperationProgress) + Send>,
//!         cancel_flag: Arc<AtomicBool>,
//!     ) -> Result<(), String> {
//!         for i in 0..self.total_steps {
//!             if cancel_flag.load(Ordering::Relaxed) {
//!                 return Err("Operation cancelled".to_string());
//!             }
//!             thread::sleep(Duration::from_millis(500));
//!             progress_callback(OperationProgress::new(
//!                 (i as f32 / self.total_steps as f32) * 100.0,
//!                 Some(format!("Step {}/{}", i + 1, self.total_steps)),
//!             ));
//!         }
//!         Ok(())
//!     }
//! }
//!
//! let manager = LongOperationManager::new();
//! let my_operation = MyOperation { total_steps: 5 };
//! let operation_id = manager.start_operation(my_operation);
//!
//! while let Some(status) = manager.get_operation_status(&operation_id) {
//!     println!("{:?}", status);
//!     thread::sleep(Duration::from_millis(100));
//! }
//! ```
//!
//! # Notes:
//!
//! - Thread-safety is ensured through the use of `Arc<Mutex<T>>` and `AtomicBool`.
//! - Operations run in their own threads, ensuring non-blocking execution.
//! - Proper cleanup of finished operations is encouraged using `cleanup_finished_operations`.
//!
//! # Definitions:
//!
//! ## `OperationStatus`
//! Represents the state of an operation. Possible states are:
//! - `Running`: Operation is ongoing.
//! - `Completed`: Operation finished successfully.
//! - `Cancelled`: Operation was cancelled by the user.
//! - `Failed(String)`: Operation failed with an error message.
//!
//! ## `OperationProgress`
//! Describes the progress of an operation, including:
//! - `percentage` (0.0 to 100.0): Indicates completion progress.
//! - `message`: Optional user-defined progress description.
//!
//! ## `LongOperation` Trait
//! Any custom long-running operation must implement this trait:
//! - `execute`: Defines the operation logic, accepting a progress callback and cancellation flag.
//!
//! ## `LongOperationManager`
//! Provides APIs to manage operations, including:
//! - `start_operation`: Starts a new operation and returns its unique ID.
//! - `get_operation_status`: Queries the current status of an operation.
//! - `get_operation_progress`: Retrieves the progress of an operation.
//! - `cancel_operation`: Cancels an operation.
//! - `cleanup_finished_operations`: Removes completed or cancelled operations to free resources.
//!
//! ## Example Operation: FileProcessingOperation
//! Represents a long-running operation to process files. Demonstrates typical usage of the framework.
//!
//! - **Fields**:
//!     - `file_path`: Path of the file to process.
//!     - `total_files`: Number of files to process.
//! - **Behavior**:
//! Simulates file processing with periodic progress updates. Supports cancellation.
//!
//!

use anyhow::Result;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;
use crate::event::{EventHub, Event, Origin, LongOperationEvent};

// Status of a long operation
#[derive(Debug, Clone, PartialEq)]
pub enum OperationStatus {
    Running,
    Completed,
    Cancelled,
    Failed(String),
}

// Progress information
#[derive(Debug, Clone)]
pub struct OperationProgress {
    pub percentage: f32, // 0.0 to 100.0
    pub message: Option<String>,
}

impl OperationProgress {
    pub fn new(percentage: f32, message: Option<String>) -> Self {
        Self {
            percentage: percentage.clamp(0.0, 100.0),
            message,
        }
    }
}

// Trait that long operations must implement
pub trait LongOperation: Send + 'static {
    type Output: Send + Sync + 'static + serde::Serialize;
    
    fn execute(
        &self,
        progress_callback: Box<dyn Fn(OperationProgress) + Send>,
        cancel_flag: Arc<AtomicBool>,
    ) -> Result<Self::Output>;
}

// Trait for operation handles (type-erased)
trait OperationHandleTrait: Send {
    fn get_status(&self) -> OperationStatus;
    fn get_progress(&self) -> OperationProgress;
    fn cancel(&self);
    fn is_finished(&self) -> bool;
}

// Concrete handle implementation
struct OperationHandle {
    status: Arc<Mutex<OperationStatus>>,
    progress: Arc<Mutex<OperationProgress>>,
    cancel_flag: Arc<AtomicBool>,
    _join_handle: thread::JoinHandle<()>,
}

impl OperationHandleTrait for OperationHandle {
    fn get_status(&self) -> OperationStatus {
        self.status.lock().unwrap().clone()
    }

    fn get_progress(&self) -> OperationProgress {
        self.progress.lock().unwrap().clone()
    }

    fn cancel(&self) {
        self.cancel_flag.store(true, Ordering::Relaxed);
        let mut status = self.status.lock().unwrap();
        if matches!(*status, OperationStatus::Running) {
            *status = OperationStatus::Cancelled;
        }
    }

    fn is_finished(&self) -> bool {
        matches!(
            self.get_status(),
            OperationStatus::Completed | OperationStatus::Cancelled | OperationStatus::Failed(_)
        )
    }
}

// Manager for long operations
pub struct LongOperationManager {
    operations: Arc<Mutex<HashMap<String, Box<dyn OperationHandleTrait>>>>,
    next_id: Arc<Mutex<u64>>,
    results: Arc<Mutex<HashMap<String, String>>>, // Store serialized results
    event_hub: Option<Arc<EventHub>>,
}

impl LongOperationManager {
    pub fn new() -> Self {
        Self {
            operations: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(0)),
            results: Arc::new(Mutex::new(HashMap::new())),
            event_hub: None,
        }
    }

    /// Inject the event hub to allow sending long operation related events
    pub fn set_event_hub(&mut self, event_hub: &Arc<EventHub>) {
        self.event_hub = Some(Arc::clone(event_hub));
    }

    /// Start a new long operation and return its ID
    pub fn start_operation<Op: LongOperation>(&self, operation: Op) -> String {
        let id = {
            let mut next_id = self.next_id.lock().unwrap();
            *next_id += 1;
            format!("op_{}", *next_id)
        };

        // Emit started event
        if let Some(event_hub) = &self.event_hub {
            event_hub.send_event(Event {
                origin: Origin::LongOperation(LongOperationEvent::Started),
                ids: vec![],
                data: Some(id.clone()),
            });
        }

        let status = Arc::new(Mutex::new(OperationStatus::Running));
        let progress = Arc::new(Mutex::new(OperationProgress::new(0.0, None)));
        let cancel_flag = Arc::new(AtomicBool::new(false));

        let status_clone = status.clone();
        let progress_clone = progress.clone();
        let cancel_flag_clone = cancel_flag.clone();
        let results_clone = self.results.clone();
        let id_clone = id.clone();
        let event_hub_opt = self.event_hub.clone();

        let join_handle = thread::spawn(move || {
            let progress_callback = {
                let progress = progress_clone.clone();
                let event_hub_opt = event_hub_opt.clone();
                let id_for_cb = id_clone.clone();
                Box::new(move |prog: OperationProgress| {
                    *progress.lock().unwrap() = prog.clone();
                    if let Some(event_hub) = &event_hub_opt {
                        let payload = serde_json::json!({
                            "id": id_for_cb,
                            "percentage": prog.percentage,
                            "message": prog.message,
                        }).to_string();
                        event_hub.send_event(Event {
                            origin: Origin::LongOperation(LongOperationEvent::Progress),
                            ids: vec![],
                            data: Some(payload),
                        });
                    }
                }) as Box<dyn Fn(OperationProgress) + Send>
            };

            let operation_result = operation.execute(progress_callback, cancel_flag_clone.clone());

            let final_status = if cancel_flag_clone.load(Ordering::Relaxed) {
                OperationStatus::Cancelled
            } else {
                match &operation_result {
                    Ok(result) => {
                        // Store the result
                        if let Ok(serialized) = serde_json::to_string(result) {
                            let mut results = results_clone.lock().unwrap();
                            results.insert(id_clone.clone(), serialized);
                        }
                        OperationStatus::Completed
                    },
                    Err(e) => OperationStatus::Failed(e.to_string()),
                }
            };

            // Emit final status event
            if let Some(event_hub) = &event_hub_opt {
                let (event, data) = match &final_status {
                    OperationStatus::Completed => (
                        LongOperationEvent::Completed,
                        serde_json::json!({"id": id_clone}).to_string(),
                    ),
                    OperationStatus::Cancelled => (
                        LongOperationEvent::Cancelled,
                        serde_json::json!({"id": id_clone}).to_string(),
                    ),
                    OperationStatus::Failed(err) => (
                        LongOperationEvent::Failed,
                        serde_json::json!({"id": id_clone, "error": err}).to_string(),
                    ),
                    OperationStatus::Running => (
                        LongOperationEvent::Progress,
                        serde_json::json!({"id": id_clone}).to_string(),
                    ),
                };
                event_hub.send_event(Event { origin: Origin::LongOperation(event), ids: vec![], data: Some(data) });
            }

            *status_clone.lock().unwrap() = final_status;
        });

        let handle = OperationHandle {
            status,
            progress,
            cancel_flag,
            _join_handle: join_handle,
        };

        self.operations
            .lock()
            .unwrap()
            .insert(id.clone(), Box::new(handle));

        id
    }

    /// Get the status of an operation
    pub fn get_operation_status(&self, id: &str) -> Option<OperationStatus> {
        let operations = self.operations.lock().unwrap();
        operations.get(id).map(|handle| handle.get_status())
    }

    /// Get the progress of an operation
    pub fn get_operation_progress(&self, id: &str) -> Option<OperationProgress> {
        let operations = self.operations.lock().unwrap();
        operations.get(id).map(|handle| handle.get_progress())
    }

    /// Cancel an operation
    pub fn cancel_operation(&self, id: &str) -> bool {
        let operations = self.operations.lock().unwrap();
        if let Some(handle) = operations.get(id) {
            handle.cancel();
            // Emit cancelled event immediately
            if let Some(event_hub) = &self.event_hub {
                let payload = serde_json::json!({"id": id}).to_string();
                event_hub.send_event(Event {
                    origin: Origin::LongOperation(LongOperationEvent::Cancelled),
                    ids: vec![],
                    data: Some(payload),
                });
            }
            true
        } else {
            false
        }
    }

    /// Check if an operation is finished
    pub fn is_operation_finished(&self, id: &str) -> Option<bool> {
        let operations = self.operations.lock().unwrap();
        operations.get(id).map(|handle| handle.is_finished())
    }

    /// Remove finished operations from memory
    pub fn cleanup_finished_operations(&self) {
        let mut operations = self.operations.lock().unwrap();
        operations.retain(|_, handle| !handle.is_finished());
    }

    /// Get list of all operation IDs
    pub fn list_operations(&self) -> Vec<String> {
        let operations = self.operations.lock().unwrap();
        operations.keys().cloned().collect()
    }

    /// Get summary of all operations
    pub fn get_operations_summary(&self) -> Vec<(String, OperationStatus, OperationProgress)> {
        let operations = self.operations.lock().unwrap();
        operations
            .iter()
            .map(|(id, handle)| (id.clone(), handle.get_status(), handle.get_progress()))
            .collect()
    }
    
    /// Store an operation result
    pub fn store_operation_result<T: serde::Serialize>(&self, id: &str, result: T) -> Result<()> {
        let serialized = serde_json::to_string(&result)?;
        let mut results = self.results.lock().unwrap();
        results.insert(id.to_string(), serialized);
        Ok(())
    }
    
    /// Get an operation result
    pub fn get_operation_result(&self, id: &str) -> Option<String> {
        let results = self.results.lock().unwrap();
        results.get(id).cloned()
    }
}

impl Default for LongOperationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::time::Duration;

    // Example implementation of a long operation
    pub struct FileProcessingOperation {
        pub file_path: String,
        pub total_files: usize,
    }

    impl LongOperation for FileProcessingOperation {
        type Output = ();
        
        fn execute(
            &self,
            progress_callback: Box<dyn Fn(OperationProgress) + Send>,
            cancel_flag: Arc<AtomicBool>,
        ) -> Result<Self::Output> {
            for i in 0..self.total_files {
                // Check if operation was cancelled
                if cancel_flag.load(Ordering::Relaxed) {
                    return Err(anyhow!("Operation was cancelled".to_string()));
                }

                // Simulate work
                thread::sleep(Duration::from_millis(500));

                // Report progress
                let percentage = (i as f32 / self.total_files as f32) * 100.0;
                progress_callback(OperationProgress::new(
                    percentage,
                    Some(format!("Processing file {} of {}", i + 1, self.total_files)),
                ));
            }

            // Final progress
            progress_callback(OperationProgress::new(100.0, Some("Completed".to_string())));
            Ok(())
        }
    }

    #[test]
    fn test_operation_manager() {
        let manager = LongOperationManager::new();

        let operation = FileProcessingOperation {
            file_path: "/tmp/test".to_string(),
            total_files: 5,
        };

        let op_id = manager.start_operation(operation);

        // Check initial status
        assert_eq!(
            manager.get_operation_status(&op_id),
            Some(OperationStatus::Running)
        );

        // Wait a bit and check progress
        thread::sleep(Duration::from_millis(100));
        let progress = manager.get_operation_progress(&op_id);
        assert!(progress.is_some());

        // Test cancellation
        assert!(manager.cancel_operation(&op_id));
        thread::sleep(Duration::from_millis(100));
        assert_eq!(
            manager.get_operation_status(&op_id),
            Some(OperationStatus::Cancelled)
        );
    }
}
