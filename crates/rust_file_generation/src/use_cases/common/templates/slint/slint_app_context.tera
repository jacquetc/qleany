use common::database::db_context::DbContext;
use common::event::EventHub;
use common::long_operation::LongOperationManager;
use common::undo_redo::UndoRedoManager;
use std::sync::{Arc, Mutex};

/// Application context that holds all shared state
#[derive(Clone)]
pub struct AppContext {
    pub db_context: DbContext,
    pub event_hub: Arc<EventHub>,
    pub quit_signal: Arc<std::sync::atomic::AtomicBool>,
    pub undo_redo_manager: Arc<Mutex<UndoRedoManager>>,
    pub long_operation_manager: Arc<Mutex<LongOperationManager>>,
}

impl AppContext {
    pub fn new() -> Self {
        let db_context = DbContext::new().expect("Failed to create database context");

        let event_hub = Arc::new(EventHub::new());
        let quit_signal = Arc::new(std::sync::atomic::AtomicBool::new(false));
        event_hub.start_event_loop(quit_signal.clone());

        let undo_redo_manager = Arc::new(Mutex::new(UndoRedoManager::new()));
        let long_operation_manager = Arc::new(Mutex::new(LongOperationManager::new()));

        // Inject event hub into long_operation_manager
        {
            let mut lom = long_operation_manager.lock().unwrap();
            lom.set_event_hub(&event_hub);
        }

        Self {
            db_context,
            event_hub,
            quit_signal,
            undo_redo_manager,
            long_operation_manager,
        }
    }

    /// Signal the event hub to stop
    pub fn shutdown(&self) {
        self.quit_signal
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

impl Default for AppContext {
    fn default() -> Self {
        Self::new()
    }
}
