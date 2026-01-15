//! File entity commands for Slint UI

use crate::app_context::AppContext;
use common::types::EntityId;
use direct_access::{file_controller, CreateFileDto, FileDto};

/// Create a new file entity
pub fn create_file(ctx: &AppContext, dto: &CreateFileDto) -> Result<FileDto, String> {
    file_controller::create(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error creating file: {:?}", e))
}

/// Create multiple file entities
pub fn create_file_multi(ctx: &AppContext, dtos: &[CreateFileDto]) -> Result<Vec<FileDto>, String> {
    file_controller::create_multi(&ctx.db_context, &ctx.event_hub, dtos)
        .map_err(|e| format!("Error creating entities: {:?}", e))
}

/// Get a file entity by ID
pub fn get_file(ctx: &AppContext, id: &EntityId) -> Result<Option<FileDto>, String> {
    file_controller::get(&ctx.db_context, id).map_err(|e| format!("Error getting file: {:?}", e))
}

/// Get multiple file entities by IDs
pub fn get_file_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<Vec<Option<FileDto>>, String> {
    file_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

/// Update a file entity
pub fn update_file(ctx: &AppContext, dto: &FileDto) -> Result<FileDto, String> {
    file_controller::update(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error updating file: {:?}", e))
}

/// Update multiple file entities
pub fn update_file_multi(ctx: &AppContext, dtos: &[FileDto]) -> Result<Vec<FileDto>, String> {
    file_controller::update_multi(&ctx.db_context, &ctx.event_hub, dtos)
        .map_err(|e| format!("Error updating entities: {:?}", e))
}

/// Remove a file entity by ID
pub fn remove_file(ctx: &AppContext, id: &EntityId) -> Result<(), String> {
    file_controller::remove(&ctx.db_context, &ctx.event_hub, id)
        .map_err(|e| format!("Error deleting file: {:?}", e))
}

/// Remove multiple file entities by IDs
pub fn remove_file_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<(), String> {
    file_controller::remove_multi(&ctx.db_context, &ctx.event_hub, ids)
        .map_err(|e| format!("Error deleting file: {:?}", e))
}
