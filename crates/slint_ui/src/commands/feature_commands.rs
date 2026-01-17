//! Feature commands for Slint UI

use crate::app_context::AppContext;
use common::direct_access::feature::FeatureRelationshipField;
use common::types::EntityId;
use direct_access::{CreateFeatureDto, FeatureDto, FeatureRelationshipDto, feature_controller};

/// Create a new feature
pub fn create_feature(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &CreateFeatureDto,
) -> Result<FeatureDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    feature_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error creating feature: {:?}", e))
}

/// Create multiple features
pub fn create_feature_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dtos: &[CreateFeatureDto],
) -> Result<Vec<FeatureDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    feature_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dtos,
    )
    .map_err(|e| format!("Error creating features: {:?}", e))
}

/// Get a feature by ID
pub fn get_feature(ctx: &AppContext, id: &EntityId) -> Result<Option<FeatureDto>, String> {
    feature_controller::get(&ctx.db_context, id)
        .map_err(|e| format!("Error getting feature: {:?}", e))
}

/// Get multiple features by IDs
pub fn get_feature_multi(
    ctx: &AppContext,
    ids: &[EntityId],
) -> Result<Vec<Option<FeatureDto>>, String> {
    feature_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting features: {:?}", e))
}

/// Update a feature
pub fn update_feature(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &FeatureDto,
) -> Result<FeatureDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    feature_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error updating feature: {:?}", e))
}

/// Update multiple features
pub fn update_feature_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dtos: &[FeatureDto],
) -> Result<Vec<FeatureDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    feature_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dtos,
    )
    .map_err(|e| format!("Error updating features: {:?}", e))
}

/// Remove a feature by ID
pub fn remove_feature(
    ctx: &AppContext,
    stack_id: Option<u64>,
    id: &EntityId,
) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    let result = feature_controller::remove(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        id,
    )
    .map_err(|e| format!("Error deleting feature: {:?}", e));

    undo_redo_manager.clear_all_stacks();
    result
}

/// Remove multiple features by IDs
pub fn remove_feature_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    ids: &[EntityId],
) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    let result = feature_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        ids,
    )
    .map_err(|e| format!("Error deleting features: {:?}", e));

    undo_redo_manager.clear_all_stacks();
    result
}

/// Get a feature relationship
pub fn get_feature_relationship(
    ctx: &AppContext,
    id: &EntityId,
    field: &FeatureRelationshipField,
) -> Result<Vec<EntityId>, String> {
    feature_controller::get_relationship(&ctx.db_context, id, field)
        .map_err(|e| format!("Error getting feature relationship: {:?}", e))
}

/// Set a feature relationship
pub fn set_feature_relationship(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &FeatureRelationshipDto,
) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    feature_controller::set_relationship(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error setting feature relationship: {:?}", e))
}
