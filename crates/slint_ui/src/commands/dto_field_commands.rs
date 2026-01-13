//! DTO Field commands for Slint UI

use crate::app_context::AppContext;
use common::types::EntityId;
use direct_access::{CreateDtoFieldDto, DtoFieldDto, dto_field_controller};

/// Create a new DTO Field
pub fn create_dto_field(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto_field: &CreateDtoFieldDto,
) -> Result<DtoFieldDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_field_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dto_field,
    )
    .map_err(|e| format!("Error creating DTO Field: {:?}", e))
}

/// Create multiple DTO Fields
pub fn create_dto_field_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto_fields: &[CreateDtoFieldDto],
) -> Result<Vec<DtoFieldDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_field_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dto_fields,
    )
    .map_err(|e| format!("Error creating DTO Fields: {:?}", e))
}

/// Get a DTO Field by ID
pub fn get_dto_field(ctx: &AppContext, id: &EntityId) -> Result<Option<DtoFieldDto>, String> {
    dto_field_controller::get(&ctx.db_context, id)
        .map_err(|e| format!("Error getting DTO Field: {:?}", e))
}

/// Get multiple DTO Fields by IDs
pub fn get_dto_field_multi(
    ctx: &AppContext,
    ids: &[EntityId],
) -> Result<Vec<Option<DtoFieldDto>>, String> {
    dto_field_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting DTO Fields: {:?}", e))
}

/// Update a DTO Field
pub fn update_dto_field(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto_field: &DtoFieldDto,
) -> Result<DtoFieldDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_field_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dto_field,
    )
    .map_err(|e| format!("Error updating DTO Field: {:?}", e))
}

/// Update multiple DTO Fields
pub fn update_dto_field_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto_fields: &[DtoFieldDto],
) -> Result<Vec<DtoFieldDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    dto_field_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dto_fields,
    )
    .map_err(|e| format!("Error updating DTO Fields: {:?}", e))
}

/// Remove a DTO Field by ID
pub fn remove_dto_field(
    ctx: &AppContext,
    stack_id: Option<u64>,
    id: &EntityId,
) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    let result = dto_field_controller::remove(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        id,
    )
    .map_err(|e| format!("Error deleting DTO Field: {:?}", e));

    undo_redo_manager.clear_all_stacks();
    result
}

/// Remove multiple DTO Fields by IDs
pub fn remove_dto_field_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    ids: &[EntityId],
) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    let result = dto_field_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        ids,
    )
    .map_err(|e| format!("Error deleting DTO Fields: {:?}", e));

    undo_redo_manager.clear_all_stacks();
    result
}
