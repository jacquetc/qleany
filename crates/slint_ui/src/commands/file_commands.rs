//! File commands for Slint UI

use crate::app_context::AppContext;
use common::types::EntityId;
use direct_access::{CreateFileDto, FileDto, file_controller};

/// Create a new file
pub fn create_file(ctx: &AppContext, dto: &CreateFileDto) -> Result<FileDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    file_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error creating file: {:?}", e))
}

/// Create multiple files
pub fn create_file_multi(ctx: &AppContext, dtos: &[CreateFileDto]) -> Result<Vec<FileDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    file_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error creating files: {:?}", e))
}

/// Get a file by ID
pub fn get_file(ctx: &AppContext, id: &EntityId) -> Result<Option<FileDto>, String> {
    file_controller::get(&ctx.db_context, id).map_err(|e| format!("Error getting file: {:?}", e))
}

/// Get multiple files by IDs
pub fn get_file_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<Vec<Option<FileDto>>, String> {
    file_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting files: {:?}", e))
}

/// Update a file
pub fn update_file(ctx: &AppContext, dto: &FileDto) -> Result<FileDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    file_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error updating file: {:?}", e))
}

/// Update multiple files
pub fn update_file_multi(ctx: &AppContext, dtos: &[FileDto]) -> Result<Vec<FileDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    file_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error updating files: {:?}", e))
}

/// Remove a file by ID
pub fn remove_file(ctx: &AppContext, id: &EntityId) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    file_controller::remove(&ctx.db_context, &ctx.event_hub, &mut *undo_redo_manager, id)
        .map_err(|e| format!("Error deleting file: {:?}", e))
}

/// Remove multiple files by IDs
pub fn remove_file_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    file_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        ids,
    )
    .map_err(|e| format!("Error deleting files: {:?}", e))
}
