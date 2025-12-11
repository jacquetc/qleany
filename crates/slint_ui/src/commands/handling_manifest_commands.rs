//! Manifest handling commands for Slint UI

use crate::app_context::AppContext;
use handling_manifest::{handling_manifest_controller, LoadDto, LoadReturnDto, SaveDto};

/// Load a manifest file
pub fn load_manifest(ctx: &AppContext, dto: &LoadDto) -> Result<LoadReturnDto, String> {
    let result = handling_manifest_controller::load(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error while loading manifest: {:?}", e))?;

    // Clear undo/redo stacks after loading
    ctx.undo_redo_manager.lock().unwrap().clear();

    Ok(result)
}

/// Save the current manifest to a file
pub fn save_manifest(ctx: &AppContext, dto: &SaveDto) -> Result<(), String> {
    handling_manifest_controller::save(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error while saving manifest: {:?}", e))?;

    // Clear undo/redo stacks after saving
    ctx.undo_redo_manager.lock().unwrap().clear();

    Ok(())
}
