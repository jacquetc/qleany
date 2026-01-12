use crate::cli::LanguageOption;
use crate::app_context::AppContext;
use direct_access::{global_controller, workspace_controller};
use handling_manifest::handling_manifest_controller;
use std::sync::Arc;
use common::direct_access::workspace::WorkspaceRelationshipField;

pub fn execute(
    app_context: &Arc<AppContext>,
    folder_path: &Option<String>,
    language: &Option<LanguageOption>,
) {
    let return_dto =
        handling_manifest_controller::new(&app_context.db_context, &app_context.event_hub)
            .map_err(|e| format!("Error while creating new manifest: {:?}", e))
            .unwrap();

    // set language if provided
    if let Some(lang) = language {
        let lang_str = match lang {
            LanguageOption::Rust => "rust",
            LanguageOption::CppQt => "cpp-qt",
        }
        .to_string();
        
        let workspace_id = return_dto.workspace_id;

        let global_id_vec = workspace_controller::get_relationship(
            &app_context.db_context,
            &workspace_id,
            &WorkspaceRelationshipField::Global,
        )
        .map_err(|e| format!("Error while getting root relationship: {:?}", e))
        .unwrap();

        let global_id = match global_id_vec.first() {
            Some(id) => id.clone(),
            None => {
                panic!("No global found for workspace id: {:?}", workspace_id);
            }
        };

        let mut global = global_controller::get(&app_context.db_context, &global_id)
            .map_err(|e| format!("Error while getting global: {:?}", e))
            .unwrap()
            .expect("Global not found");

        global.language = lang_str;
        let mut undo_redo_manager = app_context.undo_redo_manager.lock().unwrap();
        global_controller::update(
            &app_context.db_context,
            &app_context.event_hub,
            &mut *undo_redo_manager,
            None,
            &global,
        )
        .map_err(|e| format!("Error while updating global: {:?}", e))
        .unwrap();
    }

    // Determine folder path
    let folder_path = folder_path.clone().unwrap_or_else(|| ".".to_string());
    let folder_path = std::path::PathBuf::from(folder_path);
    let manifest_path = match folder_path.canonicalize() {
        Ok(canonical_path) => canonical_path.join("qleany.yaml"),
        Err(_) => folder_path.join("qleany.yaml"),
    };

    // make dirs if not exist
    if let Some(parent) = manifest_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    let dto = handling_manifest::SaveDto {
        manifest_path: manifest_path.to_string_lossy().to_string(),
    };

    handling_manifest_controller::save(&app_context.db_context, &app_context.event_hub, &dto)
        .map_err(|e| format!("Error while saving manifest: {:?}", e))
        .unwrap();
}
