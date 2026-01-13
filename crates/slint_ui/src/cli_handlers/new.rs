use crate::app_context::AppContext;
use crate::cli::{LanguageOption, NewArgs, OutputContext};
use anyhow::{bail, Result};
use common::direct_access::workspace::WorkspaceRelationshipField;
use common::types::EntityId;
use direct_access::{global_controller, workspace_controller};
use handling_manifest::handling_manifest_controller;
use std::sync::Arc;

pub fn execute(app_context: &Arc<AppContext>, args: &NewArgs, output: &OutputContext) -> Result<()> {
    let manifest_path = args.path.join("qleany.yaml");

    // Check for existing manifest
    if manifest_path.exists() && !args.force {
        bail!(
            "Manifest already exists at {}. Use --force to overwrite.",
            manifest_path.display()
        );
    }

    output.verbose(&format!(
        "Creating new manifest at {}",
        manifest_path.display()
    ));

    // Create directory structure if needed
    if let Some(parent) = manifest_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create new workspace via controller
    let return_dto =
        handling_manifest_controller::new(&app_context.db_context, &app_context.event_hub)?;
    let workspace_id = return_dto.workspace_id;

    // Apply language setting if provided
    if let Some(lang) = &args.language {
        apply_language_setting(app_context, workspace_id, lang)?;
    }

    // Apply other settings if provided
    apply_project_settings(app_context, workspace_id, args)?;

    // Save the manifest
    let save_dto = handling_manifest::SaveDto {
        manifest_path: manifest_path.to_string_lossy().to_string(),
    };
    handling_manifest_controller::save(&app_context.db_context, &app_context.event_hub, &save_dto)?;

    output.success(&format!("Created {}", manifest_path.display()));

    if output.verbose {
        output.info("Next steps:");
        output.info("  1. Edit qleany.yaml to define your entities and features");
        output.info("  2. Run 'qleany check' to validate the manifest");
        output.info("  3. Run 'qleany generate --temp' to preview generated files");
    }

    Ok(())
}

fn apply_language_setting(
    app_context: &Arc<AppContext>,
    workspace_id: EntityId,
    lang: &LanguageOption,
) -> Result<()> {
    let lang_str = match lang {
        LanguageOption::Rust => "rust",
        LanguageOption::CppQt => "cpp-qt",
    };

    let global_id = get_global_id(app_context, workspace_id)?;
    let mut global = global_controller::get(&app_context.db_context, &global_id)?
        .ok_or_else(|| anyhow::anyhow!("Global not found for id {}", global_id))?;

    global.language = lang_str.to_string();

    let mut undo_redo_manager = app_context.undo_redo_manager.lock().unwrap();
    global_controller::update(
        &app_context.db_context,
        &app_context.event_hub,
        &mut *undo_redo_manager,
        None,
        &global,
    )?;

    Ok(())
}

fn apply_project_settings(
    app_context: &Arc<AppContext>,
    workspace_id: EntityId,
    args: &NewArgs,
) -> Result<()> {
    let global_id = get_global_id(app_context, workspace_id)?;
    let mut global = global_controller::get(&app_context.db_context, &global_id)?
        .ok_or_else(|| anyhow::anyhow!("Global not found for id {}", global_id))?;

    let mut modified = false;

    if let Some(name) = &args.name {
        global.application_name = name.clone();
        modified = true;
    }
    if let Some(org_name) = &args.org_name {
        global.organisation_name = org_name.clone();
        modified = true;
    }
    if let Some(org_domain) = &args.org_domain {
        global.organisation_domain = org_domain.clone();
        modified = true;
    }

    if modified {
        let mut undo_redo_manager = app_context.undo_redo_manager.lock().unwrap();
        global_controller::update(
            &app_context.db_context,
            &app_context.event_hub,
            &mut *undo_redo_manager,
            None,
            &global,
        )?;
    }

    Ok(())
}

fn get_global_id(app_context: &Arc<AppContext>, workspace_id: EntityId) -> Result<EntityId> {
    let global_ids = workspace_controller::get_relationship(
        &app_context.db_context,
        &workspace_id,
        &WorkspaceRelationshipField::Global,
    )?;

    global_ids
        .first()
        .copied()
        .ok_or_else(|| anyhow::anyhow!("No global found for workspace {}", workspace_id))
}
