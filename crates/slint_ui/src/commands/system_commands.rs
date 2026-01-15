//! System entity commands for Slint UI

use crate::app_context::AppContext;
use common::direct_access::system::SystemRelationshipField;
use common::types::EntityId;
use direct_access::SystemRelationshipDto;
use direct_access::{system_controller, CreateSystemDto, SystemDto};

/// Create a new system entity
pub fn create_system(ctx: &AppContext, dto: &CreateSystemDto) -> Result<SystemDto, String> {
    system_controller::create(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error creating system: {:?}", e))
}

/// Create multiple system entities
pub fn create_system_multi(
    ctx: &AppContext,

    dtos: &[CreateSystemDto],
) -> Result<Vec<SystemDto>, String> {
    system_controller::create_multi(&ctx.db_context, &ctx.event_hub, dtos)
        .map_err(|e| format!("Error creating entities: {:?}", e))
}

/// Get a system entity by ID
pub fn get_system(ctx: &AppContext, id: &EntityId) -> Result<Option<SystemDto>, String> {
    system_controller::get(&ctx.db_context, id)
        .map_err(|e| format!("Error getting system: {:?}", e))
}

/// Get multiple system entities by IDs
pub fn get_system_multi(
    ctx: &AppContext,
    ids: &[EntityId],
) -> Result<Vec<Option<SystemDto>>, String> {
    system_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

/// Update a system entity
pub fn update_system(ctx: &AppContext, dto: &SystemDto) -> Result<SystemDto, String> {
    system_controller::update(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error updating system: {:?}", e))
}

/// Update multiple system entities
pub fn update_system_multi(ctx: &AppContext, dtos: &[SystemDto]) -> Result<Vec<SystemDto>, String> {
    system_controller::update_multi(&ctx.db_context, &ctx.event_hub, dtos)
        .map_err(|e| format!("Error updating entities: {:?}", e))
}

/// Remove a system entity by ID
pub fn remove_system(ctx: &AppContext, id: &EntityId) -> Result<(), String> {
    system_controller::remove(&ctx.db_context, &ctx.event_hub, id)
        .map_err(|e| format!("Error deleting system: {:?}", e))
}

/// Remove multiple system entities by IDs
pub fn remove_system_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<(), String> {
    system_controller::remove_multi(&ctx.db_context, &ctx.event_hub, ids)
        .map_err(|e| format!("Error deleting system: {:?}", e))
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

    dto: &SystemRelationshipDto,
) -> Result<(), String> {
    system_controller::set_relationship(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error setting system relationship: {:?}", e))
}
