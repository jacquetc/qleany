//! Relationship commands for Slint UI

use crate::app_context::AppContext;
use common::direct_access::relationship::RelationshipRelationshipField;
use common::types::EntityId;
use direct_access::{
    CreateRelationshipDto, RelationshipDto, RelationshipRelationshipDto, relationship_controller,
};

/// Create a new relationship
pub fn create_relationship(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &CreateRelationshipDto,
) -> Result<RelationshipDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    relationship_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error creating relationship: {:?}", e))
}

/// Create multiple relationships
pub fn create_relationship_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dtos: &[CreateRelationshipDto],
) -> Result<Vec<RelationshipDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    relationship_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dtos,
    )
    .map_err(|e| format!("Error creating relationships: {:?}", e))
}

/// Get a relationship by ID
pub fn get_relationship(
    ctx: &AppContext,
    id: &EntityId,
) -> Result<Option<RelationshipDto>, String> {
    relationship_controller::get(&ctx.db_context, id)
        .map_err(|e| format!("Error getting relationship: {:?}", e))
}

/// Get multiple relationships by IDs
pub fn get_relationship_multi(
    ctx: &AppContext,
    ids: &[EntityId],
) -> Result<Vec<Option<RelationshipDto>>, String> {
    relationship_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting relationships: {:?}", e))
}

/// Update a relationship
pub fn update_relationship(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &RelationshipDto,
) -> Result<RelationshipDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    relationship_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error updating relationship: {:?}", e))
}

/// Update multiple relationships
pub fn update_relationship_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dtos: &[RelationshipDto],
) -> Result<Vec<RelationshipDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    relationship_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dtos,
    )
    .map_err(|e| format!("Error updating relationships: {:?}", e))
}

/// Remove a relationship by ID
pub fn remove_relationship(
    ctx: &AppContext,
    stack_id: Option<u64>,
    id: &EntityId,
) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    relationship_controller::remove(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        id,
    )
    .map_err(|e| format!("Error deleting relationship: {:?}", e))
}

/// Remove multiple relationships by IDs
pub fn remove_relationship_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    ids: &[EntityId],
) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    relationship_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        ids,
    )
    .map_err(|e| format!("Error deleting relationships: {:?}", e))
}

/// Get a relationship's relationship
pub fn get_relationship_relationship(
    ctx: &AppContext,
    id: &EntityId,
    field: &RelationshipRelationshipField,
) -> Result<Vec<EntityId>, String> {
    relationship_controller::get_relationship(&ctx.db_context, id, field)
        .map_err(|e| format!("Error getting relationship relationship: {:?}", e))
}

/// Set a relationship's relationship
pub fn set_relationship_relationship(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &RelationshipRelationshipDto,
) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    relationship_controller::set_relationship(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error setting relationship relationship: {:?}", e))
}
