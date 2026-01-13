use crate::app_context::AppContext;
use crate::cli::{ExportArgs, ExportFormat, OutputContext};
use anyhow::Result;
use handling_manifest::handling_manifest_controller;
use std::path::Path;
use std::sync::Arc;

pub fn execute(
    app_context: &Arc<AppContext>,
    manifest_path: &Path,
    args: &ExportArgs,
    output: &OutputContext,
) -> Result<()> {
    // Load manifest
    let load_dto = handling_manifest::LoadDto {
        manifest_path: manifest_path.to_string_lossy().to_string(),
    };
    handling_manifest_controller::load(&app_context.db_context, &app_context.event_hub, &load_dto)?;

    output.verbose(&format!("Exporting from {}", manifest_path.display()));

    let content = match &args.format {
        ExportFormat::Mermaid => export_mermaid(app_context)?,
        ExportFormat::Json => export_json(manifest_path)?,
    };

    // Write to file or stdout
    if let Some(output_path) = &args.output {
        std::fs::write(output_path, &content)?;
        output.success(&format!("Exported to {}", output_path.display()));
    } else {
        println!("{}", content);
    }

    Ok(())
}

fn export_mermaid(app_context: &Arc<AppContext>) -> Result<String> {
    let result = handling_manifest_controller::export_to_mermaid(
        &app_context.db_context,
        &app_context.event_hub,
    )?;

    Ok(result.mermaid_diagram)
}

fn export_json(manifest_path: &Path) -> Result<String> {
    let yaml_content = std::fs::read_to_string(manifest_path)?;
    let yaml: serde_yml::Value = serde_yml::from_str(&yaml_content)?;
    let json = serde_json::to_string_pretty(&yaml)?;
    Ok(json)
}
