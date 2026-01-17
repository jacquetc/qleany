//! UserInterface entity commands for Slint UI

use crate::app_context::AppContext;
use common::types::EntityId;
use direct_access::{CreateUserInterfaceDto, UserInterfaceDto, user_interface_controller};

/// Create a new user interface
pub fn create_user_interface(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &CreateUserInterfaceDto,
) -> Result<UserInterfaceDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    user_interface_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error creating user interface: {:?}", e))
}

/// Get a user interface by ID
pub fn get_user_interface(
    ctx: &AppContext,
    id: &EntityId,
) -> Result<Option<UserInterfaceDto>, String> {
    user_interface_controller::get(&ctx.db_context, id)
        .map_err(|e| format!("Error getting user interface: {:?}", e))
}

/// Update a user interface
pub fn update_user_interface(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &UserInterfaceDto,
) -> Result<UserInterfaceDto, String> {
    let mut undo_redo_manager = ctx.undo_redo_manager.lock().unwrap();
    user_interface_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dto,
    )
    .map_err(|e| format!("Error updating user interface: {:?}", e))
}

/// Get multiple user interfaces by IDs
pub fn get_user_interface_multi(
    ctx: &AppContext,
    ids: &[EntityId],
) -> Result<Vec<Option<UserInterfaceDto>>, String> {
    user_interface_controller::get_multi(&ctx.db_context, ids)
        .map_err(|e| format!("Error getting user interfaces: {:?}", e))
}
