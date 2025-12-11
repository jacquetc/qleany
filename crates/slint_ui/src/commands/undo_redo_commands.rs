//! Undo/Redo commands for Slint UI

use crate::app_context::AppContext;

/// Undoes the most recent command.
pub fn undo(ctx: &AppContext) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    // Inject event hub before operation
    undo_redo_manager.set_event_hub(&ctx.event_hub);
    
    undo_redo_manager.undo().map_err(|e| e.to_string())
}

/// Redoes the most recently undone command.
pub fn redo(ctx: &AppContext) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    undo_redo_manager.set_event_hub(&ctx.event_hub);
    
    undo_redo_manager.redo().map_err(|e| e.to_string())
}

/// Checks if there are commands that can be undone.
pub fn can_undo(ctx: &AppContext) -> bool {
    let undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    undo_redo_manager.can_undo()
}

/// Checks if there are commands that can be redone.
pub fn can_redo(ctx: &AppContext) -> bool {
    let undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    undo_redo_manager.can_redo()
}

/// Begins a composite command group.
pub fn begin_composite(ctx: &AppContext) {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    undo_redo_manager.set_event_hub(&ctx.event_hub);
    undo_redo_manager.begin_composite();
}

/// Ends the current composite command group.
pub fn end_composite(ctx: &AppContext) {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    undo_redo_manager.set_event_hub(&ctx.event_hub);
    undo_redo_manager.end_composite();
}
