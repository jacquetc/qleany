//! System commands for Slint UI

use common::direct_access::system::SystemRelationshipField;
use crate::app_context::AppContext;
use common::types::EntityId;
use direct_access::{CreateSystemDto, SystemDto, system_controller, SystemRelationshipDto};

/// Create a new system
pub fn create_system(ctx: &AppContext, dto: &CreateSystemDto) -> Result<SystemDto, String> {
    system_controller::create(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error creating system: {:?}", e))
}

/// Create multiple systems
pub fn create_system_multi(ctx: &AppContext, dtos: &[CreateSystemDto]) -> Result<Vec<SystemDto>, String> {
    system_controller::create_multi(&ctx.db_context, &ctx.event_hub, dtos)
        .map_err(|e| format!("Error creating systems: {:?}", e))
}

/// Get a system by ID
pub fn get_system(ctx: &AppContext, id: &EntityId) -> Result<Option<SystemDto>, String> {
    system_controller::get(&ctx.db_context, id).map_err(|e| format!("Error getting system: {:?}", e))
}

/// Get multiple systems by IDs
pub fn get_system_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<Vec<Option<SystemDto>>, String> {
    system_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting systems: {:?}", e))
}

/// Update a system
pub fn update_system(ctx: &AppContext, dto: &SystemDto) -> Result<SystemDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    system_controller::update(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error updating system: {:?}", e))
}

/// Update multiple systems
pub fn update_system_multi(ctx: &AppContext, dtos: &[SystemDto]) -> Result<Vec<SystemDto>, String> {
    system_controller::update_multi(&ctx.db_context, &ctx.event_hub, dtos)
        .map_err(|e| format!("Error updating systems: {:?}", e))
}

/// Remove a system by ID
pub fn remove_system(ctx: &AppContext, id: &EntityId) -> Result<(), String> {
    system_controller::remove(&ctx.db_context, &ctx.event_hub, id)
        .map_err(|e| format!("Error deleting system: {:?}", e))
}

/// Remove multiple systems by IDs
pub fn remove_system_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<(), String> {
    system_controller::remove_multi(&ctx.db_context, &ctx.event_hub, ids)
        .map_err(|e| format!("Error deleting systems: {:?}", e))
}

/// Get a system relationship
pub fn get_system_relationship(
    ctx: &AppContext,
    id: &EntityId,
    field: &SystemRelationshipField,
) -> Result<Vec<EntityId>, String> {
    system_controller::get_relationship(&ctx.db_context, id, field)
        .map_err(|e| format!("Error getting system relationship: {:?}", e))
}

/// Set a system relationship
pub fn set_system_relationship(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &SystemRelationshipDto,
) -> Result<(), String> {
    system_controller::set_relationship(
        &ctx.db_context,
        &ctx.event_hub,
        dto,
    )
        .map_err(|e| format!("Error setting system relationship: {:?}", e))
}
