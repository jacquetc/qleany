//! Field commands for Slint UI

use crate::app_context::AppContext;
use common::direct_access::field::FieldRelationshipField;
use common::types::EntityId;
use direct_access::{field_controller, CreateFieldDto, FieldDto, FieldRelationshipDto};

/// Create a new field
pub fn create_field(ctx: &AppContext, dto: &CreateFieldDto) -> Result<FieldDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    field_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error creating field: {:?}", e))
}

/// Create multiple fields
pub fn create_field_multi(ctx: &AppContext, dtos: &[CreateFieldDto]) -> Result<Vec<FieldDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    field_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error creating fields: {:?}", e))
}

/// Get a field by ID
pub fn get_field(ctx: &AppContext, id: &EntityId) -> Result<Option<FieldDto>, String> {
    field_controller::get(&ctx.db_context, id)
        .map_err(|e| format!("Error getting field: {:?}", e))
}

/// Get multiple fields by IDs
pub fn get_field_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<Vec<Option<FieldDto>>, String> {
    field_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting fields: {:?}", e))
}

/// Update a field
pub fn update_field(ctx: &AppContext, dto: &FieldDto) -> Result<FieldDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    field_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error updating field: {:?}", e))
}

/// Update multiple fields
pub fn update_field_multi(ctx: &AppContext, dtos: &[FieldDto]) -> Result<Vec<FieldDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    field_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error updating fields: {:?}", e))
}

/// Remove a field by ID
pub fn remove_field(ctx: &AppContext, id: &EntityId) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    field_controller::remove(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        id,
    )
    .map_err(|e| format!("Error deleting field: {:?}", e))
}

/// Remove multiple fields by IDs
pub fn remove_field_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    field_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        ids,
    )
    .map_err(|e| format!("Error deleting fields: {:?}", e))
}

/// Get a field relationship
pub fn get_field_relationship(
    ctx: &AppContext,
    id: &EntityId,
    field: &FieldRelationshipField,
) -> Result<Vec<EntityId>, String> {
    field_controller::get_relationship(&ctx.db_context, id, field)
        .map_err(|e| format!("Error getting field relationship: {:?}", e))
}

/// Set a field relationship
pub fn set_field_relationship(ctx: &AppContext, dto: &FieldRelationshipDto) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    field_controller::set_relationship(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error setting field relationship: {:?}", e))
}
