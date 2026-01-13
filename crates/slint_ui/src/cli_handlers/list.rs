mod file_tree;

use crate::app_context::AppContext;
use crate::cli::{ListArgs, ListTarget, OutputContext, OutputFormat};
use anyhow::Result;
use common::direct_access::system::SystemRelationshipField;
use direct_access::{system_controller, EntityDto, UseCaseDto};
use handling_manifest::handling_manifest_controller;
use rust_file_generation::rust_file_generation_controller;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use common::entities::Entity;

/// The root system entity ID (singleton in the database)
const ROOT_SYSTEM_ID: u64 = 1;

pub fn execute(
    app_context: &Arc<AppContext>,
    manifest_path: &Path,
    args: &ListArgs,
    output: &OutputContext,
) -> Result<()> {
    // Load manifest first
    let load_dto = handling_manifest::LoadDto {
        manifest_path: manifest_path.to_string_lossy().to_string(),
    };
    handling_manifest_controller::load(&app_context.db_context, &app_context.event_hub, &load_dto)?;

    let target = args.target.as_ref().unwrap_or(&ListTarget::Files);

    match target {
        ListTarget::Files => list_files(app_context, args, output),
        ListTarget::Entities => list_entities(app_context, args, output),
        ListTarget::Features => list_features(app_context, args, output),
        ListTarget::Groups => list_groups(app_context, args),
    }
}

fn list_files(
    app_context: &Arc<AppContext>,
    args: &ListArgs,
    output: &OutputContext,
) -> Result<()> {
    let dto = rust_file_generation::ListRustFilesDto {
        only_list_already_existing: args.existing_only,
    };

    let result = rust_file_generation_controller::list_rust_files(
        &app_context.db_context,
        &app_context.event_hub,
        &dto,
    )?;

    match args.format {
        OutputFormat::Plain => {
            for file in &result.file_names {
                println!("{}", file);
            }
            output.info(&format!("\n{} files", result.file_names.len()));
        }
        OutputFormat::Json => {
            let json = serde_json::json!({
                "files": result.file_names,
                "count": result.file_names.len()
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Tree => {
            file_tree::print_file_tree(&result.file_names);
        }
    }

    Ok(())
}

fn list_entities(
    app_context: &Arc<AppContext>,
    args: &ListArgs,
    output: &OutputContext,
) -> Result<()> {
    use common::direct_access::workspace::WorkspaceRelationshipField;
    use direct_access::{entity_controller, workspace_controller};

    // Get workspace and entities
    let workspaces = workspace_controller::get_multi(&app_context.db_context, &vec![])?;
    let workspace = workspaces
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No workspace loaded"))?
        .ok_or_else(|| anyhow::anyhow!("Workspace data is empty"))?;

    let entity_ids = workspace_controller::get_relationship(
        &app_context.db_context,
        &workspace.id,
        &WorkspaceRelationshipField::Entities,
    )?;

    let entities = entity_controller::get_multi(&app_context.db_context, entity_ids.as_slice())?
    .into_iter().filter_map(|x| x).collect::<Vec<EntityDto>>();

    match args.format {
        OutputFormat::Plain => {
            for entity in &entities {
                let heritage = if entity.only_for_heritage {
                    " (heritage only)"
                } else {
                    ""
                };
                println!("{}{}", entity.name, heritage);
            }
            output.info(&format!("\n{} entities", entities.len()));
        }
        OutputFormat::Json => {
            let json: Vec<_> = entities
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "name": e.name,
                        "only_for_heritage": e.only_for_heritage,
                        "undoable": e.undoable,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Tree => {
            file_tree::print_file_tree(&entities.iter().map(|e| e.name.as_str()).collect::<Vec<&str>>());
        }
    }

    Ok(())
}

fn list_features(
    app_context: &Arc<AppContext>,
    args: &ListArgs,
    output: &OutputContext,
) -> Result<()> {
    use common::direct_access::feature::FeatureRelationshipField;
    use common::direct_access::workspace::WorkspaceRelationshipField;
    use direct_access::{feature_controller, use_case_controller, workspace_controller};

    let workspaces = workspace_controller::get_multi(&app_context.db_context, &vec![])?;
    let workspace = workspaces
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No workspace loaded"))?
        .ok_or_else(|| anyhow::anyhow!("Workspace data is empty"))?;

    let feature_ids = workspace_controller::get_relationship(
        &app_context.db_context,
        &workspace.id,
        &WorkspaceRelationshipField::Features,
    )?;

    let mut features_data = Vec::new();
    for feature_id in feature_ids {
        if let Some(feature) = feature_controller::get(&app_context.db_context, &feature_id)? {
            let use_case_ids = feature_controller::get_relationship(
                &app_context.db_context,
                &feature_id,
                &FeatureRelationshipField::UseCases,
            )?;

            let mut use_cases = Vec::new();
            for uc_id in use_case_ids {
                if let Some(uc) = use_case_controller::get(&app_context.db_context, &uc_id)? {
                    use_cases.push(uc);
                }
            }
            features_data.push((feature, use_cases));
        }
    }

    match args.format {
        OutputFormat::Plain => {
            for (feature, use_cases) in &features_data {
                println!("{}:", feature.name);
                for uc in use_cases {
                    let flags = format_use_case_flags(uc);
                    println!("  - {}{}", uc.name, flags);
                }
            }
            output.info(&format!(
                "\n{} features, {} use cases",
                features_data.len(),
                features_data.iter().map(|(_, ucs)| ucs.len()).sum::<usize>()
            ));
        }
        OutputFormat::Json => {
            let json: Vec<_> = features_data
                .iter()
                .map(|(f, ucs)| {
                    serde_json::json!({
                        "name": f.name,
                        "use_cases": ucs.iter().map(|uc| serde_json::json!({
                            "name": uc.name,
                            "undoable": uc.undoable,
                            "read_only": uc.read_only,
                            "validator": uc.validator,
                        })).collect::<Vec<_>>()
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Tree => {

        }
    }

    Ok(())
}

fn list_groups(app_context: &Arc<AppContext>, args: &ListArgs) -> Result<()> {
    use direct_access::file_controller;

    let file_ids = system_controller::get_relationship(
        &app_context.db_context,
        &ROOT_SYSTEM_ID,
        &SystemRelationshipField::Files,
    )?;
    let mut groups: BTreeMap<String, usize> = BTreeMap::new();

    for id in file_ids {
        if let Some(file) = file_controller::get(&app_context.db_context, &id)? {
            *groups.entry(file.group).or_insert(0) += 1;
        }
    }

    match args.format {
        OutputFormat::Plain | OutputFormat::Tree => {
            for (group, count) in &groups {
                println!("{} ({} files)", group, count);
            }
        }
        OutputFormat::Json => {
            let json: Vec<_> = groups
                .iter()
                .map(|(g, c)| serde_json::json!({"name": g, "file_count": c}))
                .collect();
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
    }

    Ok(())
}

fn format_use_case_flags(uc: &UseCaseDto) -> String {
    let mut flags = Vec::new();
    if uc.undoable {
        flags.push("undoable");
    }
    if uc.read_only {
        flags.push("read-only");
    }
    if uc.long_operation {
        flags.push("async");
    }
    if flags.is_empty() {
        String::new()
    } else {
        format!(" [{}]", flags.join(", "))
    }
}