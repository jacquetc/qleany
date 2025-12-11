//! Entity commands for Slint UI

use crate::app_context::AppContext;
use common::direct_access::entity::EntityRelationshipField;
use common::types::EntityId;
use direct_access::{entity_controller, CreateEntityDto, EntityDto, EntityRelationshipDto};

/// Create a new entity
pub fn create_entity(ctx: &AppContext, dto: &CreateEntityDto) -> Result<EntityDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    entity_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error creating entity: {:?}", e))
}

/// Create multiple entities
pub fn create_entity_multi(ctx: &AppContext, dtos: &[CreateEntityDto]) -> Result<Vec<EntityDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    entity_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

/// Get an entity by ID
pub fn get_entity(ctx: &AppContext, id: &EntityId) -> Result<Option<EntityDto>, String> {
    entity_controller::get(&ctx.db_context, id)
        .map_err(|e| format!("Error getting entity: {:?}", e))
}

/// Get multiple entities by IDs
pub fn get_entity_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<Vec<Option<EntityDto>>, String> {
    entity_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

/// Update an entity
pub fn update_entity(ctx: &AppContext, dto: &EntityDto) -> Result<EntityDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    entity_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error updating entity: {:?}", e))
}

/// Update multiple entities
pub fn update_entity_multi(ctx: &AppContext, dtos: &[EntityDto]) -> Result<Vec<EntityDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    entity_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

/// Remove an entity by ID
pub fn remove_entity(ctx: &AppContext, id: &EntityId) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    entity_controller::remove(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        id,
    )
    .map_err(|e| format!("Error deleting entity: {:?}", e))
}

/// Remove multiple entities by IDs
pub fn remove_entity_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    entity_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        ids,
    )
    .map_err(|e| format!("Error deleting entities: {:?}", e))
}

/// Get an entity relationship
pub fn get_entity_relationship(
    ctx: &AppContext,
    id: &EntityId,
    field: &EntityRelationshipField,
) -> Result<Vec<EntityId>, String> {
    entity_controller::get_relationship(&ctx.db_context, id, field)
        .map_err(|e| format!("Error getting entity relationship: {:?}", e))
}

/// Set an entity relationship
pub fn set_entity_relationship(ctx: &AppContext, dto: &EntityRelationshipDto) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    entity_controller::set_relationship(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error setting entity relationship: {:?}", e))
}
