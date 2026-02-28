use crate::app_context::AppContext;
use crate::cli::OutputContext;
use anyhow::Result;
use handling_manifest::handling_manifest_controller;
use std::path::Path;
use std::sync::Arc;

pub fn execute(
    app_context: &Arc<AppContext>,
    manifest_path: &Path,
    output: &OutputContext,
) -> Result<()> {
    output.verbose(&format!("Upgrading {}", manifest_path.display()));

    // Load (triggers automatic migration to current schema version)
    let load_dto = handling_manifest::LoadDto {
        manifest_path: manifest_path.to_string_lossy().to_string(),
    };

    handling_manifest_controller::load(
        &app_context.db_context,
        &app_context.event_hub,
        &load_dto,
    )?;

    // Save back with current schema version
    let save_dto = handling_manifest::SaveDto {
        manifest_path: manifest_path.to_string_lossy().to_string(),
    };

    handling_manifest_controller::save(
        &app_context.db_context,
        &app_context.event_hub,
        &save_dto,
    )?;

    output.success(&format!("Upgraded {}", manifest_path.display()));

    Ok(())
}
