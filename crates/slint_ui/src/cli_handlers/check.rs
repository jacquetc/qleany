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
    output.verbose(&format!("Validating {}", manifest_path.display()));

    let load_dto = handling_manifest::LoadDto {
        manifest_path: manifest_path.to_string_lossy().to_string(),
    };

    // Load validates the manifest structure
    let result = handling_manifest_controller::load(
        &app_context.db_context,
        &app_context.event_hub,
        &load_dto,
    )?;

    output.success(&format!(
        "Manifest is valid (workspace_id: {})",
        result.workspace_id
    ));

    // TODO: Additional validation could go here:
    // - Check for orphaned entity references
    // - Validate relationship consistency

    Ok(())
}
