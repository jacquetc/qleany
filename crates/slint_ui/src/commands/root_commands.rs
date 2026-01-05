//! Root entity commands for Slint UI

use crate::app_context::AppContext;
use common::direct_access::root::RootRelationshipField;
use common::types::EntityId;
use direct_access::{CreateRootDto, RootDto, RootRelationshipDto, root_controller};

/// Create a new root entity
pub fn create_root(ctx: &AppContext, dto: &CreateRootDto) -> Result<RootDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    root_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error creating root: {:?}", e))
}

/// Create multiple root entities
pub fn create_root_multi(ctx: &AppContext, dtos: &[CreateRootDto]) -> Result<Vec<RootDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    root_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

/// Get a root entity by ID
pub fn get_root(ctx: &AppContext, id: &EntityId) -> Result<Option<RootDto>, String> {
    root_controller::get(&ctx.db_context, id).map_err(|e| format!("Error getting root: {:?}", e))
}

/// Get multiple root entities by IDs
pub fn get_root_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<Vec<Option<RootDto>>, String> {
    root_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

/// Update a root entity
pub fn update_root(ctx: &AppContext, dto: &RootDto) -> Result<RootDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    root_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error updating root: {:?}", e))
}

/// Update multiple root entities
pub fn update_root_multi(ctx: &AppContext, dtos: &[RootDto]) -> Result<Vec<RootDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    root_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

/// Remove a root entity by ID
pub fn remove_root(ctx: &AppContext, id: &EntityId) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    root_controller::remove(&ctx.db_context, &ctx.event_hub, &mut *undo_redo_manager, id)
        .map_err(|e| format!("Error deleting root: {:?}", e))
}

/// Remove multiple root entities by IDs
pub fn remove_root_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    root_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        ids,
    )
    .map_err(|e| format!("Error deleting entities: {:?}", e))
}

/// Get a root relationship
pub fn get_root_relationship(
    ctx: &AppContext,
    id: &EntityId,
    field: &RootRelationshipField,
) -> Result<Vec<EntityId>, String> {
    root_controller::get_relationship(&ctx.db_context, id, field)
        .map_err(|e| format!("Error getting root relationship: {:?}", e))
}

/// Set a root relationship
pub fn set_root_relationship(ctx: &AppContext, dto: &RootRelationshipDto) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    root_controller::set_relationship(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error setting root relationship: {:?}", e))
}
