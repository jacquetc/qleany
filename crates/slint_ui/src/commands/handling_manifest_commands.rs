//! Manifest handling commands for Slint UI

use crate::app_context::AppContext;
use handling_manifest::{
    LoadDto, LoadReturnDto, NewReturnDto, SaveDto, handling_manifest_controller,
};

/// Load a manifest file
pub fn load_manifest(ctx: &AppContext, dto: &LoadDto) -> Result<LoadReturnDto, String> {
    let result = handling_manifest_controller::load(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error while loading manifest: {:?}", e))?;

    // Clear undo/redo stacks after loading
    ctx.undo_redo_manager.lock().unwrap().clear_all_stacks();

    Ok(result)
}

/// Save the current manifest to a file
pub fn save_manifest(ctx: &AppContext, dto: &SaveDto) -> Result<(), String> {
    handling_manifest_controller::save(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error while saving manifest: {:?}", e))?;

    // Clear undo/redo stacks after saving
    ctx.undo_redo_manager.lock().unwrap().clear_all_stacks();

    Ok(())
}

/// New manifest
pub fn new_manifest(ctx: &AppContext) -> Result<NewReturnDto, String> {
    let result = handling_manifest_controller::new(&ctx.db_context, &ctx.event_hub)
        .map_err(|e| format!("Error while creating new manifest: {:?}", e))?;
    ctx.undo_redo_manager.lock().unwrap().clear_all_stacks();
    Ok(result)
}

/// Close the current manifest
pub fn close_manifest(ctx: &AppContext) -> Result<(), String> {
    let _result = handling_manifest_controller::close(&ctx.db_context, &ctx.event_hub)
        .map_err(|e| format!("Error while closing manifest: {:?}", e))?;

    ctx.undo_redo_manager.lock().unwrap().clear_all_stacks();
    Ok(())
}
