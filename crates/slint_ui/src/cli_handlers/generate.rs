use crate::app_context::AppContext;
use crate::cli::{GenerateArgs, GenerateTarget, OutputContext};
use anyhow::Result;
use common::long_operation::OperationStatus;
use common::types::EntityId;
use handling_manifest::handling_manifest_controller;
use rust_file_generation::rust_file_generation_controller;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub fn execute(
    app_context: &Arc<AppContext>,
    manifest_path: &Path,
    args: &GenerateArgs,
    output: &OutputContext,
) -> Result<()> {
    // Load manifest
    let load_dto = handling_manifest::LoadDto {
        manifest_path: manifest_path.to_string_lossy().to_string(),
    };
    handling_manifest_controller::load(&app_context.db_context, &app_context.event_hub, &load_dto)?;

    // Determine output path
    let output_path = determine_output_path(app_context, args)?;

    if args.dry_run {
        output.info("Dry run - no files will be written");
    }

    output.verbose(&format!("Output directory: {}", output_path.display()));

    // Get file IDs to generate based on target
    let file_ids = collect_file_ids(app_context, args)?;

    if file_ids.is_empty() {
        output.warn("No files match the specified criteria");
        return Ok(());
    }

    output.info(&format!("Generating {} files...", file_ids.len()));

    if args.dry_run {
        list_files_to_generate(app_context, &file_ids, output)?;
        return Ok(());
    }

    // Perform generation
    let prefix = get_prefix_path(app_context)?;
    let dto = rust_file_generation::GenerateRustFilesDto {
        file_ids: file_ids.iter().map(|&id| id as EntityId).collect(),
        root_path: output_path.to_string_lossy().to_string(),
        prefix,
    };

    let operation_id = {
        let mut long_op_manager = app_context.long_operation_manager.lock().unwrap();
        rust_file_generation_controller::generate_rust_files(
            &app_context.db_context,
            &app_context.event_hub,
            &mut long_op_manager,
            &dto,
        )?
    };

    // Set up SIGINT handler for graceful cancellation
    let cancelled = Arc::new(AtomicBool::new(false));
    let cancelled_clone = cancelled.clone();

    // Attempt to set up Ctrl+C handler; if it fails, proceed without cancellation support
    let ctrlc_result = ctrlc::set_handler(move || {
        cancelled_clone.store(true, Ordering::SeqCst);
    });

    if ctrlc_result.is_err() {
        output.verbose("Note: Ctrl+C cancellation not available");
    }

    // Poll for completion
    let mut last_percentage: f32 = 0.0;

    loop {
        std::thread::sleep(Duration::from_millis(100));

        // Check for user cancellation request
        if cancelled.load(Ordering::SeqCst) {
            output.info("\nCancellation requested, stopping generation...");
            let long_op_manager = app_context.long_operation_manager.lock().unwrap();
            long_op_manager.cancel_operation(&operation_id);
            // Continue loop to wait for actual cancellation status
        }

        let long_op_manager = app_context.long_operation_manager.lock().unwrap();

        // Check operation status
        let status = match long_op_manager.get_operation_status(&operation_id) {
            Some(s) => s,
            None => {
                output.warn("Operation not found");
                break;
            }
        };

        // Report progress if verbose
        if output.verbose
            && let Some(progress) = long_op_manager.get_operation_progress(&operation_id) {
                // Only report if percentage changed significantly
                if (progress.percentage - last_percentage).abs() >= 5.0 {
                    let msg = progress.message.as_deref().unwrap_or("");
                    output.verbose(&format!("[{:.0}%] {}", progress.percentage, msg));
                    last_percentage = progress.percentage;
                }
            }

        // Handle terminal states
        match status {
            OperationStatus::Running => {
                // Continue polling
            }
            OperationStatus::Completed => {
                // Retrieve and display result
                if let Some(result_json) = long_op_manager.get_operation_result(&operation_id) {
                    drop(long_op_manager); // Release lock before parsing
                    handle_completed_result(&result_json, output)?;
                } else {
                    output.success("Generation completed");
                }
                break;
            }
            OperationStatus::Cancelled => {
                output.warn("Generation was cancelled");
                break;
            }
            OperationStatus::Failed(err) => {
                anyhow::bail!("Generation failed: {}", err);
            }
        }
    }

    Ok(())
}

fn handle_completed_result(result_json: &str, output: &OutputContext) -> Result<()> {
    let result: rust_file_generation::GenerateRustFilesReturnDto =
        serde_json::from_str(result_json)?;

    output.success(&format!(
        "Generated {} files ({})",
        result.files.len(),
        result.timestamp
    ));

    if output.verbose {
        for file in &result.files {
            output.verbose(&format!("  {}", file));
        }
    }

    Ok(())
}

fn determine_output_path(app_context: &Arc<AppContext>, args: &GenerateArgs) -> Result<PathBuf> {
    if args.temp {
        let temp_path = std::env::current_dir()?.join("temp");
        std::fs::create_dir_all(&temp_path)?;
        return Ok(temp_path);
    }

    if let Some(output) = &args.output {
        std::fs::create_dir_all(output)?;
        return Ok(output.clone());
    }

    // Use manifest's prefix_path relative to current directory
    let prefix = get_prefix_path(app_context)?;
    let path = std::env::current_dir()?.join(&prefix);
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

fn get_prefix_path(app_context: &Arc<AppContext>) -> Result<String> {
    use common::direct_access::workspace::WorkspaceRelationshipField;
    use direct_access::{global_controller, workspace_controller};

    let workspaces = workspace_controller::get_multi(&app_context.db_context, &[])?;
    let workspace = workspaces
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No workspace loaded"))?
        .ok_or_else(|| anyhow::anyhow!("Workspace data is empty"))?;

    let global_ids = workspace_controller::get_relationship(
        &app_context.db_context,
        &workspace.id,
        &WorkspaceRelationshipField::Global,
    )?;
    let global_id = global_ids
        .first()
        .ok_or_else(|| anyhow::anyhow!("No global configuration found"))?;

    let global = global_controller::get(&app_context.db_context, global_id)?
        .ok_or_else(|| anyhow::anyhow!("Global configuration not found"))?;

    Ok(global.prefix_path)
}

fn collect_file_ids(app_context: &Arc<AppContext>, args: &GenerateArgs) -> Result<Vec<EntityId>> {
    use direct_access::file_controller;

    // First, list all files
    let dto = rust_file_generation::ListRustFilesDto {
        only_list_already_existing: false,
    };
    let list_result = rust_file_generation_controller::list_rust_files(
        &app_context.db_context,
        &app_context.event_hub,
        &dto,
    )?;

    let target = args.target.as_ref().unwrap_or(&GenerateTarget::All);

    match target {
        GenerateTarget::All => Ok(list_result.file_ids),

        GenerateTarget::Feature { name } => {
            filter_files_by_path_prefix(app_context, &list_result.file_ids, name)
        }

        GenerateTarget::Entity { name } => {
            filter_files_by_path_prefix(app_context, &list_result.file_ids, name)
        }

        GenerateTarget::Group { name } => {
            let mut matching = Vec::new();
            for id in list_result.file_ids {
                if let Some(file) = file_controller::get(&app_context.db_context, &id)?
                    && file.group.eq_ignore_ascii_case(name) {
                        matching.push(id);
                    }
            }
            Ok(matching)
        }

        GenerateTarget::File { target } => {
            // Try to parse as numeric ID first
            if let Ok(id) = target.parse::<EntityId>()
                && list_result.file_ids.contains(&id) {
                    return Ok(vec![id]);
                }

            // Otherwise match by path
            for (idx, file_path) in list_result.file_names.iter().enumerate() {
                if file_path.ends_with(target) || file_path == target {
                    return Ok(vec![list_result.file_ids[idx]]);
                }
            }

            anyhow::bail!("File not found: {}", target);
        }
    }
}

fn filter_files_by_path_prefix(
    app_context: &Arc<AppContext>,
    file_ids: &[EntityId],
    prefix: &str,
) -> Result<Vec<EntityId>> {
    use direct_access::file_controller;

    let prefix_lower = prefix.to_lowercase();
    let mut matching = Vec::new();

    for &id in file_ids {
        if let Some(file) = file_controller::get(&app_context.db_context, &id)? {
            let path_lower = file.relative_path.to_lowercase();
            if path_lower.contains(&prefix_lower) {
                matching.push(id);
            }
        }
    }

    Ok(matching)
}

fn list_files_to_generate(
    app_context: &Arc<AppContext>,
    file_ids: &[EntityId],
    output: &OutputContext,
) -> Result<()> {
    use direct_access::file_controller;

    output.info("Files that would be generated:");
    for &id in file_ids {
        if let Some(file) = file_controller::get(&app_context.db_context, &id)? {
            println!("  {}", file.relative_path);
        }
    }
    Ok(())
}
