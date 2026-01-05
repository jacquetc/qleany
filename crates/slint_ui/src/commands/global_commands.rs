//! Global commands for Slint UI

use crate::app_context::AppContext;
use common::types::EntityId;
use direct_access::{CreateGlobalDto, GlobalDto, global_controller};

/// Create a new global
pub fn create_global(ctx: &AppContext, dto: &CreateGlobalDto) -> Result<GlobalDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    global_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error creating global: {:?}", e))
}

/// Create multiple globals
pub fn create_global_multi(
    ctx: &AppContext,
    dtos: &[CreateGlobalDto],
) -> Result<Vec<GlobalDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    global_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error creating globals: {:?}", e))
}

/// Get a global by ID
pub fn get_global(ctx: &AppContext, id: &EntityId) -> Result<Option<GlobalDto>, String> {
    global_controller::get(&ctx.db_context, id)
        .map_err(|e| format!("Error getting global: {:?}", e))
}

/// Get multiple globals by IDs
pub fn get_global_multi(
    ctx: &AppContext,
    ids: &[EntityId],
) -> Result<Vec<Option<GlobalDto>>, String> {
    global_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting globals: {:?}", e))
}

/// Update a global
pub fn update_global(ctx: &AppContext, dto: &GlobalDto) -> Result<GlobalDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    global_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error updating global: {:?}", e))
}

/// Update multiple globals
pub fn update_global_multi(ctx: &AppContext, dtos: &[GlobalDto]) -> Result<Vec<GlobalDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    global_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error updating globals: {:?}", e))
}

/// Remove a global by ID
pub fn remove_global(ctx: &AppContext, id: &EntityId) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    global_controller::remove(&ctx.db_context, &ctx.event_hub, &mut *undo_redo_manager, id)
        .map_err(|e| format!("Error deleting global: {:?}", e))
}

/// Remove multiple globals by IDs
pub fn remove_global_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    global_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        ids,
    )
    .map_err(|e| format!("Error deleting globals: {:?}", e))
}
