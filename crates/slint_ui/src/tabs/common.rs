use std::sync::Arc;
use common::types::EntityId;
use crate::app_context::AppContext;
use crate::commands::workspace_commands;
use crate::{App, AppState};
use slint::ComponentHandle;

/// Helper function to get the global_id from root
pub fn get_global_id(app: &App, app_context: &Arc<AppContext>) -> Option<EntityId> {
    let workspace_id = app.global::<AppState>().get_workspace_id() as EntityId;
    if workspace_id > 0
        && let Ok(Some(workspace)) = workspace_commands::get_workspace(app_context, &workspace_id)
        && workspace.global > 0
    {
        log::trace!("Found global_id: {}", workspace.global);
        return Some(workspace.global);
    }
    None
}