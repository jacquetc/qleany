//! Workspace entity commands for Slint UI

use crate::app_context::AppContext;
use common::direct_access::workspace::WorkspaceRelationshipField;
use common::types::EntityId;
use direct_access::{CreateWorkspaceDto, WorkspaceDto, WorkspaceRelationshipDto, workspace_controller};

/// Create a new workspace entity
pub fn create_workspace(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &CreateWorkspaceDto,
) -> Result<WorkspaceDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    workspace_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error creating workspace: {:?}", e))
}

/// Create multiple workspace entities
pub fn create_workspace_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dtos: &[CreateWorkspaceDto],
) -> Result<Vec<WorkspaceDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    workspace_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dtos,
    )
    .map_err(|e| format!("Error creating entities: {:?}", e))
}

/// Get a workspace entity by ID
pub fn get_workspace(ctx: &AppContext, id: &EntityId) -> Result<Option<WorkspaceDto>, String> {
    workspace_controller::get(&ctx.db_context, id).map_err(|e| format!("Error getting workspace: {:?}", e))
}

/// Get multiple workspace entities by IDs
pub fn get_workspace_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<Vec<Option<WorkspaceDto>>, String> {
    workspace_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting entities: {:?}", e))
}

/// Update a workspace entity
pub fn update_workspace(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &WorkspaceDto,
) -> Result<WorkspaceDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    workspace_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error updating workspace: {:?}", e))
}

/// Update multiple workspace entities
pub fn update_workspace_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dtos: &[WorkspaceDto],
) -> Result<Vec<WorkspaceDto>, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    workspace_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dtos,
    )
    .map_err(|e| format!("Error updating entities: {:?}", e))
}

/// Remove a workspace entity by ID
pub fn remove_workspace(ctx: &AppContext, stack_id: Option<u64>, id: &EntityId) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    let result = workspace_controller::remove(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        id,
    )
    .map_err(|e| format!("Error deleting workspace: {:?}", e));

    undo_redo_manager.clear_all_stacks();
    result
}

/// Remove multiple workspace entities by IDs
pub fn remove_workspace_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    ids: &[EntityId],
) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    let result = workspace_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        ids,
    )
    .map_err(|e| format!("Error deleting workspace: {:?}", e));

    undo_redo_manager.clear_all_stacks();
    result
}

/// Get a workspace relationship
pub fn get_workspace_relationship(
    ctx: &AppContext,
    id: &EntityId,
    field: &WorkspaceRelationshipField,
) -> Result<Vec<EntityId>, String> {
    workspace_controller::get_relationship(&ctx.db_context, id, field)
        .map_err(|e| format!("Error getting workspace relationship: {:?}", e))
}

/// Set a workspace relationship
pub fn set_workspace_relationship(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &WorkspaceRelationshipDto,
) -> Result<(), String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    workspace_controller::set_relationship(
        &ctx.db_context,
        &ctx.event_hub,
        &mut *undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error setting workspace relationship: {:?}", e))
}
