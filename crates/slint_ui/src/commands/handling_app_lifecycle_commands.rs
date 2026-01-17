//! Manifest handling commands for Slint UI

use crate::app_context::AppContext;
use handling_app_lifecycle::handling_app_lifecycle_controller;

/// Load a manifest file
pub fn initialize_app(ctx: &AppContext) -> Result<(), String> {
    handling_app_lifecycle_controller::initialize_app(&ctx.db_context, &ctx.event_hub)
        .map_err(|e| format!("Error while initializing app: {:?}", e))?;

    Ok(())
}

pub fn clean_up_before_exit(ctx: &AppContext) -> Result<(), String> {
    handling_app_lifecycle_controller::clean_up_before_exit(&ctx.db_context, &ctx.event_hub)
        .map_err(|e| format!("Error while cleaning up before exit: {:?}", e))?;

    Ok(())
}
