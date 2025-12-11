//! Home Tab module
//!
//! This module contains the logic specific to the Home tab,
//! including manifest-related callback handlers (new, open, save, close, open_qleany).

use std::sync::Arc;

use slint::ComponentHandle;
use common::direct_access::root::RootRelationshipField;
use handling_manifest::LoadDto;

use crate::app_context::AppContext;
use crate::commands::{entity_commands, handling_manifest_commands, root_commands};
use crate::{App, EntitiesTabState, AppState, ListItem, ManifestCommands};

/// Wire up the on_new_manifest callback
pub fn setup_new_manifest_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ManifestCommands>().on_new_manifest({
        let ctx = Arc::clone(app_context);
        move || {
            log::info!("New Manifest clicked");
            // TODO: Implement new manifest logic
            let _ = ctx; // Use context when implementing
        }
    });
}

/// Wire up the on_open_manifest callback
pub fn setup_open_manifest_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ManifestCommands>().on_open_manifest({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("Open Manifest clicked");
            // TODO: Open file dialog and load manifest
            // For now, demonstrate the command pattern:
            let load_dto = LoadDto {
                manifest_path: "qleany.yaml".to_string(),
            };
            match handling_manifest_commands::load_manifest(&ctx, &load_dto) {
                Ok(result) => {
                    log::info!("Manifest loaded successfully: {:?}", result);
                    if let Some(app) = app_weak.upgrade() {
                        app.global::<AppState>().set_error_message(slint::SharedString::from(""));
                    }
                }
                Err(e) => {
                    log::error!("Failed to load manifest: {}", e);
                    if let Some(app) = app_weak.upgrade() {
                        app.global::<AppState>().set_error_message(slint::SharedString::from(e));
                    }
                }
            }
        }
    });
}

/// Wire up the on_save_manifest callback
pub fn setup_save_manifest_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ManifestCommands>().on_save_manifest({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("Save Manifest clicked");
            let save_dto = handling_manifest::SaveDto {
                manifest_path: "qleany.yaml".to_string(),
            };
            match handling_manifest_commands::save_manifest(&ctx, &save_dto) {
                Ok(()) => {
                    log::info!("Manifest saved successfully");
                    if let Some(app) = app_weak.upgrade() {
                        app.global::<AppState>().set_error_message(slint::SharedString::from(""));
                    }
                }
                Err(e) => {
                    log::error!("Failed to save manifest: {}", e);
                    if let Some(app) = app_weak.upgrade() {
                        app.global::<AppState>().set_error_message(slint::SharedString::from(e));
                    }
                }
            }
        }
    });
}

/// Wire up the on_close_manifest callback
pub fn setup_close_manifest_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ManifestCommands>().on_close_manifest({
        let ctx = Arc::clone(app_context);
        move || {
            log::info!("Close Manifest clicked");
            // TODO: Clear the current manifest from context
            let _ = ctx;
        }
    });
}

/// Wire up the on_open_qleany_manifest callback
pub fn setup_open_qleany_manifest_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ManifestCommands>().on_open_qleany_manifest({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("Open Qleany Manifest clicked");
            // Load the qleany.yaml from the project root
            let load_dto = LoadDto {
                manifest_path: "qleany.yaml".to_string(),
            };
            match handling_manifest_commands::load_manifest(&ctx, &load_dto) {
                Ok(result) => {
                    log::info!("Qleany manifest loaded: {:?}", result);
                    if let Some(app) = app_weak.upgrade() {
                        // clear any previous error
                        app.global::<AppState>().set_error_message(slint::SharedString::from(""));

                        // set root_id
                        app.global::<AppState>().set_root_id(result.root_id as i32);

                        // 1) Get entities attached to the root
                        let root_id = result.root_id as common::types::EntityId;
                        let entity_ids_res = root_commands::get_root_relationship(
                            &ctx,
                            &root_id,
                            &RootRelationshipField::Entities,
                        );

                        match entity_ids_res {
                            Ok(entity_ids) => {
                                // 2) Fetch entities details to obtain names
                                match entity_commands::get_entity_multi(&ctx, &entity_ids) {
                                    Ok(entities_opt) => {
                                        // Map to ListItem (id + text)
                                        let mut list: Vec<ListItem> = Vec::new();
                                        for maybe_entity in entities_opt.into_iter() {
                                            if let Some(e) = maybe_entity {
                                                list.push(ListItem {
                                                    id: e.id as i32,
                                                    text: slint::SharedString::from(e.name),
                                                    subtitle: slint::SharedString::from(""),
                                                    checked: false,
                                                });
                                            }
                                        }

                                        // 3) Apply to AppState
                                        let model = std::rc::Rc::new(slint::VecModel::from(list));
                                        app.global::<EntitiesTabState>().set_entity_cr_list(model.into());

                                        // Reset selections related to entities/fields
                                        app.global::<EntitiesTabState>().set_selected_entity_id(-1);
                                        app.global::<EntitiesTabState>().set_selected_entity_name(slint::SharedString::from(""));
                                        let list: Vec<ListItem> = Vec::new();
                                        let model = std::rc::Rc::new(slint::VecModel::from(list));
                                        app.global::<EntitiesTabState>().set_field_cr_list(model.into());
                                        app.global::<EntitiesTabState>().set_selected_field_id(-1);
                                    }
                                    Err(e) => {
                                        log::error!("Failed to fetch entities: {}", e);
                                        app.global::<AppState>().set_error_message(slint::SharedString::from(e));
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to get root entities: {}", e);
                                app.global::<AppState>().set_error_message(slint::SharedString::from(e));
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to load qleany manifest: {}", e);
                    if let Some(app) = app_weak.upgrade() {
                        app.global::<AppState>().set_error_message(slint::SharedString::from(e));
                    }
                }
            }
        }
    });
}

/// Initialize all home tab related callbacks
pub fn init(app: &App, app_context: &Arc<AppContext>) {
    setup_new_manifest_callback(app, app_context);
    setup_open_manifest_callback(app, app_context);
    setup_save_manifest_callback(app, app_context);
    setup_close_manifest_callback(app, app_context);
    setup_open_qleany_manifest_callback(app, app_context);
}
