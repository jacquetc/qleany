//! DTO commands for Slint UI

use crate::app_context::AppContext;
use common::direct_access::dto::DtoRelationshipField;
use common::types::EntityId;
use direct_access::{dto_controller, CreateDtoDto, DtoDto, DtoRelationshipDto};

/// Create a new DTO
pub fn create_dto(ctx: &AppContext, dto: &CreateDtoDto) -> Result<DtoDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error creating DTO: {:?}", e))
}

/// Create multiple DTOs
pub fn create_dto_multi(ctx: &AppContext, dtos: &[CreateDtoDto]) -> Result<Vec<DtoDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error creating DTOs: {:?}", e))
}

/// Get a DTO by ID
pub fn get_dto(ctx: &AppContext, id: &EntityId) -> Result<Option<DtoDto>, String> {
    dto_controller::get(&ctx.db_context, id)
        .map_err(|e| format!("Error getting DTO: {:?}", e))
}

/// Get multiple DTOs by IDs
pub fn get_dto_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<Vec<Option<DtoDto>>, String> {
    dto_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting DTOs: {:?}", e))
}

/// Update a DTO
pub fn update_dto(ctx: &AppContext, dto: &DtoDto) -> Result<DtoDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error updating DTO: {:?}", e))
}

/// Update multiple DTOs
pub fn update_dto_multi(ctx: &AppContext, dtos: &[DtoDto]) -> Result<Vec<DtoDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error updating DTOs: {:?}", e))
}

/// Remove a DTO by ID
pub fn remove_dto(ctx: &AppContext, id: &EntityId) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_controller::remove(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        id,
    )
    .map_err(|e| format!("Error deleting DTO: {:?}", e))
}

/// Remove multiple DTOs by IDs
pub fn remove_dto_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        ids,
    )
    .map_err(|e| format!("Error deleting DTOs: {:?}", e))
}

/// Get a DTO relationship
pub fn get_dto_relationship(
    ctx: &AppContext,
    id: &EntityId,
    field: &DtoRelationshipField,
) -> Result<Vec<EntityId>, String> {
    dto_controller::get_relationship(&ctx.db_context, id, field)
        .map_err(|e| format!("Error getting DTO relationship: {:?}", e))
}

/// Set a DTO relationship
pub fn set_dto_relationship(ctx: &AppContext, dto: &DtoRelationshipDto) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_controller::set_relationship(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error setting DTO relationship: {:?}", e))
}
