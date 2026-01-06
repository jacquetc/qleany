//! Home Tab module
//!
//! This module contains the logic specific to the Home tab,
//! including manifest-related callback handlers (new, open, save, close, open_qleany).

use std::sync::Arc;

use common::event::{HandlingManifestEvent, Origin};
use handling_manifest::LoadDto;
use slint::ComponentHandle;

use crate::app_context::AppContext;
use crate::commands::{handling_manifest_commands, root_commands};
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, ManifestCommands};
use slint::Timer;

fn subscribe_loaded_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(Origin::HandlingManifest(HandlingManifestEvent::Load), {
        let ctx = Arc::clone(&app_context);
        let app_weak = app.as_weak();
        move |event| {
            log::info!("Manifest loaded event received: {:?}", event);
            let _ctx = Arc::clone(&ctx);
            let app_weak = app_weak.clone();

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak.upgrade() {
                    // clear any previous error
                    app.global::<AppState>()
                        .set_error_message(slint::SharedString::from(""));
                    // set loading
                    app.global::<AppState>().set_is_loading(true);
                    // set root_id
                    app.global::<AppState>().set_root_id(event.ids[0] as i32);

                    app.global::<AppState>().set_manifest_is_saved(true);
                    app.global::<AppState>().set_is_loading(false);
                    app.global::<AppState>().set_manifest_is_open(true);
                    log::info!("Manifest UI state updated after load");
                }
            });
        }
    });
}

fn subscribe_closed_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(Origin::HandlingManifest(HandlingManifestEvent::Close), {
        let ctx = Arc::clone(&app_context);
        let app_weak = app.as_weak();
        move |event| {
            log::info!("Manifest closed event received: {:?}", event);
            let _ctx = Arc::clone(&ctx);
            let app_weak = app_weak.clone();

            // Use invoke_from_event_loop to safely update UI from background thread
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak.upgrade() {
                    // clear any previous error
                    app.global::<AppState>()
                        .set_error_message(slint::SharedString::from(""));
                    // set root_id
                    app.global::<AppState>().set_root_id(-1);

                    app.global::<AppState>().set_manifest_is_saved(true);
                    app.global::<AppState>().set_is_loading(false);
                    app.global::<AppState>().set_manifest_is_open(false);
                }
            });
        }
    });
}

fn subscribe_new_manifest_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(
        Origin::HandlingManifest(HandlingManifestEvent::New),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("New manifest created event received");
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        // clear any previous error
                        app.global::<AppState>()
                            .set_error_message(slint::SharedString::from(""));
                        // set root_id
                        app.global::<AppState>().set_root_id(event.ids[0] as i32);

                        app.global::<AppState>().set_manifest_is_saved(true);
                        app.global::<AppState>().set_is_loading(false);
                        app.global::<AppState>().set_manifest_is_open(true);
                    }
                });
            }
        },
    );
}


/// Wire up the on_new_manifest callback
pub fn setup_new_manifest_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ManifestCommands>().on_new_manifest({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("New Manifest clicked");

            if let Some(app) = app_weak.upgrade() {
                if app.global::<AppState>().get_manifest_is_open() {
                    log::info!("A manifest is already open, closing it first");
                    // Close any currently open manifest first
                    match handling_manifest_commands::close_manifest(&ctx) {
                        Ok(()) => {
                            log::info!("Manifest closed successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to close manifest: {}", e);
                            return;
                        }
                    }
                }
                // set loading
                app.global::<AppState>().set_is_loading(true);

                match handling_manifest_commands::new_manifest(&ctx) {
                    Ok(result) => {
                        log::info!("New manifest created successfully");
                    }
                    Err(e) => {
                        log::error!("Failed to create new manifest: {}", e);
                        app.global::<AppState>().set_is_loading(false);
                        return;
                    }
                }

            }
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

            // Get the user's home directory as the default path
            let default_path = dirs::home_dir().unwrap_or_default();

            // Open file dialog using rfd
            let file_dialog = rfd::FileDialog::new()
                .add_filter("YAML files", &["yaml", "yml"])
                .set_directory(&default_path)
                .set_file_name("qleany.yaml");

            if let Some(app) = app_weak.upgrade() {
                if let Some(path) = file_dialog.pick_file() {
                    let manifest_path = path.to_string_lossy().to_string();
                    log::info!("Selected manifest file: {}", manifest_path);

                    if app.global::<AppState>().get_manifest_is_open() {
                        log::info!("A manifest is already open, closing it first");

                        // Close any currently open manifest first
                        match handling_manifest_commands::close_manifest(&ctx) {
                            Ok(()) => {
                                log::info!("Manifest closed successfully");
                            }
                            Err(e) => {
                                log::error!("Failed to close manifest: {}", e);
                                return;
                            }
                        }
                    }
                    let load_dto = LoadDto { manifest_path };
                    match handling_manifest_commands::load_manifest(&ctx, &load_dto) {
                        Ok(result) => {
                            log::info!("Manifest loaded successfully: {:?}", result);
                        }

                        Err(e) => {
                            log::error!("Failed to load manifest: {}", e);
                            if let Some(app) = app_weak.upgrade() {
                                app.global::<AppState>().set_is_loading(false);
                                app.global::<AppState>()
                                    .set_error_message(slint::SharedString::from(e));
                            }
                        }
                    }
                }
            } else {
                log::info!("File dialog cancelled");
            }
        }
    });
}

/// Wire up the on_save_manifest callback
pub fn setup_save_manifest_as_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ManifestCommands>().on_save_manifest_as({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("Save Manifest As clicked");

            // Get the user's home directory as the default path
            let default_path = dirs::home_dir().unwrap_or_default();

            // Open save file dialog using rfd
            let file_dialog = rfd::FileDialog::new()
                .add_filter("YAML files", &["yaml", "yml"])
                .set_directory(&default_path)
                .set_file_name("qleany.yaml");

            if let Some(path) = file_dialog.save_file() {
                let manifest_path = path.to_string_lossy().to_string();
                log::info!("Selected save path: {}", manifest_path);

                let save_dto = handling_manifest::SaveDto { manifest_path };
                match handling_manifest_commands::save_manifest(&ctx, &save_dto) {
                    Ok(()) => {
                        log::info!("Manifest saved successfully");
                        if let Some(app) = app_weak.upgrade() {
                            app.global::<AppState>()
                                .set_error_message(slint::SharedString::from(""));
                            app.global::<AppState>().set_manifest_is_saved(true);
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to save manifest: {}", e);
                        if let Some(app) = app_weak.upgrade() {
                            app.global::<AppState>()
                                .set_error_message(slint::SharedString::from(e));
                        }
                    }
                }
            } else {
                log::info!("Save file dialog cancelled");
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

            let root_id = app_weak
                .upgrade()
                .map_or(0, |app| app.global::<AppState>().get_root_id() as u64);
            if root_id < 1 {
                log::error!("No manifest is currently loaded, cannot save");
                return;
            }
            let root = root_commands::get_root(&ctx, &root_id as &common::types::EntityId)
                .ok()
                .flatten();
            if root.is_none() {
                log::error!("Failed to get root entity for id {}", root_id);
                return;
            }
            let manifest_absolute_path = root.unwrap().manifest_absolute_path;
            log::info!("Saving manifest to path: {}", manifest_absolute_path);

            // add "qleany.yaml" if manifest_absolute_path is a directory
            let manifest_absolute_path = {
                let path = std::path::Path::new(&manifest_absolute_path);
                if path.is_dir() {
                    path.join("qleany.yaml").to_string_lossy().to_string()
                } else {
                    manifest_absolute_path
                }
            };

            let save_dto = handling_manifest::SaveDto {
                manifest_path: manifest_absolute_path,
            };
            match handling_manifest_commands::save_manifest(&ctx, &save_dto) {
                Ok(()) => {
                    log::info!("Manifest saved successfully");
                    if let Some(app) = app_weak.upgrade() {
                        app.global::<AppState>()
                            .set_error_message(slint::SharedString::from(""));
                        app.global::<AppState>().set_manifest_is_saved(true);
                    }
                }
                Err(e) => {
                    log::error!("Failed to save manifest: {}", e);
                    if let Some(app) = app_weak.upgrade() {
                        app.global::<AppState>()
                            .set_error_message(slint::SharedString::from(e));
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
        let app_weak = app.as_weak();
        move || {
            log::info!("Close Manifest clicked");
            match handling_manifest_commands::close_manifest(&ctx) {
                Ok(()) => {
                    log::info!("Manifest closed successfully");
                    if let Some(app) = app_weak.upgrade() {
                        app.global::<AppState>().set_manifest_is_saved(true);
                        app.global::<AppState>().set_manifest_is_open(false);
                    }
                }
                Err(e) => {
                    log::error!("Failed to close manifest: {}", e);
                }
            }
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
            if let Some(app) = app_weak.upgrade() {
                if app.global::<AppState>().get_manifest_is_open() {
                    log::info!("A manifest is already open, closing it first");

                    // Close any currently open manifest first
                    match handling_manifest_commands::close_manifest(&ctx) {
                        Ok(()) => {
                            log::info!("Manifest closed successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to close manifest: {}", e);
                        }
                    }
                }

                // Load the qleany.yaml from the project root
                let load_dto = LoadDto {
                    manifest_path: "qleany.yaml".to_string(),
                };
                match handling_manifest_commands::load_manifest(&ctx, &load_dto) {
                    Ok(result) => {
                        log::info!("Qleany manifest loaded: {:?}", result);
                        // clear any previous error
                        app.global::<AppState>()
                            .set_error_message(slint::SharedString::from(""));

                        // set root_id
                        app.global::<AppState>().set_root_id(result.root_id as i32);

                        // set loading
                        app.global::<AppState>().set_is_loading(true);
                        Timer::single_shot(std::time::Duration::from_millis(100), move || {
                            app.global::<AppState>().set_manifest_is_saved(true);
                            app.global::<AppState>().set_is_loading(false);
                            app.global::<AppState>().set_manifest_is_open(true);
                        });
                    }
                    Err(e) => {
                        log::error!("Failed to load qleany manifest: {}", e);
                        if let Some(app) = app_weak.upgrade() {
                            app.global::<AppState>()
                                .set_error_message(slint::SharedString::from(e));
                        }
                    }
                }
            }
        }
    });
}

/// Initialize all home tab related callbacks
pub fn init(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    subscribe_loaded_event(event_hub_client, app, app_context);
    subscribe_closed_event(event_hub_client, app, app_context);
    subscribe_new_manifest_event(event_hub_client, app, app_context);
    setup_new_manifest_callback(app, app_context);
    setup_open_manifest_callback(app, app_context);
    setup_save_manifest_callback(app, app_context);
    setup_save_manifest_as_callback(app, app_context);
    setup_close_manifest_callback(app, app_context);
    setup_open_qleany_manifest_callback(app, app_context);
}
