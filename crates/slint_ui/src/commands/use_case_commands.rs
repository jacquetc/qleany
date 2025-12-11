//! Use case commands for Slint UI

use crate::app_context::AppContext;
use common::direct_access::use_case::UseCaseRelationshipField;
use common::types::EntityId;
use direct_access::{use_case_controller, CreateUseCaseDto, UseCaseDto, UseCaseRelationshipDto};

/// Create a new use case
pub fn create_use_case(ctx: &AppContext, dto: &CreateUseCaseDto) -> Result<UseCaseDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    use_case_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error creating use case: {:?}", e))
}

/// Create multiple use cases
pub fn create_use_case_multi(ctx: &AppContext, dtos: &[CreateUseCaseDto]) -> Result<Vec<UseCaseDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    use_case_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error creating use cases: {:?}", e))
}

/// Get a use case by ID
pub fn get_use_case(ctx: &AppContext, id: &EntityId) -> Result<Option<UseCaseDto>, String> {
    use_case_controller::get(&ctx.db_context, id)
        .map_err(|e| format!("Error getting use case: {:?}", e))
}

/// Get multiple use cases by IDs
pub fn get_use_case_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<Vec<Option<UseCaseDto>>, String> {
    use_case_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting use cases: {:?}", e))
}

/// Update a use case
pub fn update_use_case(ctx: &AppContext, dto: &UseCaseDto) -> Result<UseCaseDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    use_case_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error updating use case: {:?}", e))
}

/// Update multiple use cases
pub fn update_use_case_multi(ctx: &AppContext, dtos: &[UseCaseDto]) -> Result<Vec<UseCaseDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    use_case_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dtos,
    )
    .map_err(|e| format!("Error updating use cases: {:?}", e))
}

/// Remove a use case by ID
pub fn remove_use_case(ctx: &AppContext, id: &EntityId) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    use_case_controller::remove(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        id,
    )
    .map_err(|e| format!("Error deleting use case: {:?}", e))
}

/// Remove multiple use cases by IDs
pub fn remove_use_case_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    use_case_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        ids,
    )
    .map_err(|e| format!("Error deleting use cases: {:?}", e))
}

/// Get a use case relationship
pub fn get_use_case_relationship(
    ctx: &AppContext,
    id: &EntityId,
    field: &UseCaseRelationshipField,
) -> Result<Vec<EntityId>, String> {
    use_case_controller::get_relationship(&ctx.db_context, id, field)
        .map_err(|e| format!("Error getting use case relationship: {:?}", e))
}

/// Set a use case relationship
pub fn set_use_case_relationship(ctx: &AppContext, dto: &UseCaseRelationshipDto) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    use_case_controller::set_relationship(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        dto,
    )
    .map_err(|e| format!("Error setting use case relationship: {:?}", e))
}
