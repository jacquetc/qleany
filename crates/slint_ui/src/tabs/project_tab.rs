//! Project Tab module
//!
//! This module contains the logic specific to the Project tab,
//! including callback handlers for project settings management.

use std::sync::Arc;

use crate::app_context::AppContext;
use crate::commands::{global_commands, workspace_commands};
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, ProjectTabState};
use common::event::{DirectAccessEntity, EntityEvent, HandlingManifestEvent, Origin};
use slint::ComponentHandle;

fn create_new_undo_stack(app: &App, app_context: &Arc<AppContext>) {
    let ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        let stack_id = ctx.undo_redo_manager.lock().unwrap().create_new_stack();
        log::info!("New undo stack created with ID: {}", stack_id);
        app.global::<ProjectTabState>()
            .set_project_undo_stack_id(stack_id as i32);
    }
}

fn delete_undo_stack(app: &App, app_context: &Arc<AppContext>) {
    let ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        let stack_id = app.global::<ProjectTabState>().get_project_undo_stack_id() as u64;
        let result = ctx.undo_redo_manager.lock().unwrap().delete_stack(stack_id);
        match result {
            Ok(()) => {
                log::info!("Undo stack with ID {} deleted", stack_id);
                app.global::<ProjectTabState>()
                    .set_project_undo_stack_id(-1);
            }
            Err(e) => {
                log::error!("Failed to delete undo stack {}: {}", stack_id, e);
            }
        }
    }
}

/// Fill the ProjectTabState with data from the Global entity
fn fill_project_tab(app: &App, app_context: &Arc<AppContext>) {
    log::info!("Filling ProjectTabState with data from Global entity");

    if let Some(global_id) = get_global_id(app, app_context) {
        if let Ok(Some(global)) = global_commands::get_global(app_context, &global_id) {
            log::info!("Filling ProjectTabState with global data: {:?}", global);
            let language = match global.language.to_lowercase().as_str() {
                "rust" => "Rust",
                "cpp-qt" => "C++ / Qt",
                _ => "Rust",
            };
            app.global::<ProjectTabState>()
                .set_language(slint::SharedString::from(language));
            app.global::<ProjectTabState>()
                .set_application_name(slint::SharedString::from(&global.application_name));
            app.global::<ProjectTabState>()
                .set_organisation_name(slint::SharedString::from(&global.organisation_name));
            app.global::<ProjectTabState>()
                .set_organisation_domain(slint::SharedString::from(&global.organisation_domain));
            app.global::<ProjectTabState>()
                .set_prefix_path(slint::SharedString::from(&global.prefix_path));
        }
    }
}

fn clear_project_tab(app: &App) {
    log::info!("Clearing ProjectTabState data");
    app.global::<ProjectTabState>()
        .set_language(slint::SharedString::from("Rust"));
    app.global::<ProjectTabState>()
        .set_application_name(slint::SharedString::from(""));
    app.global::<ProjectTabState>()
        .set_organisation_name(slint::SharedString::from(""));
    app.global::<ProjectTabState>()
        .set_organisation_domain(slint::SharedString::from(""));
    app.global::<ProjectTabState>()
        .set_prefix_path(slint::SharedString::from(""));
}

fn subscribe_close_manifest_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(Origin::HandlingManifest(HandlingManifestEvent::Close), {
        let ctx = Arc::clone(&app_context);
        let app_weak = app.as_weak();
        move |event| {
            log::info!("Manifest closed event received: {:?}", event);
            let ctx = Arc::clone(&ctx);
            let app_weak = app_weak.clone();

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak.upgrade() {
                    if app.global::<AppState>().get_manifest_is_open() {
                        clear_project_tab(&app);
                        delete_undo_stack(&app, &ctx);
                    }
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
    event_hub_client.subscribe(Origin::HandlingManifest(HandlingManifestEvent::New), {
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |_event| {
            log::info!("New manifest created event received");
            let ctx = Arc::clone(&ctx);
            let app_weak = app_weak.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak.upgrade() {
                    if app.global::<AppState>().get_manifest_is_open() {
                        fill_project_tab(&app, &ctx);
                        create_new_undo_stack(&app, &ctx);
                    }
                }
            });
        }
    });
}

fn subscribe_load_manifest_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(Origin::HandlingManifest(HandlingManifestEvent::Load), {
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |_event| {
            log::info!("Manifest loaded event received");
            let ctx = Arc::clone(&ctx);
            let app_weak = app_weak.clone();

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak.upgrade() {
                    if app.global::<AppState>().get_manifest_is_open() {
                        fill_project_tab(&app, &ctx);
                        create_new_undo_stack(&app, &ctx);
                    }
                }
            });
        }
    });
}

/// Subscribe to Global created events to populate ProjectTabState when manifest is loaded
fn subscribe_global_created_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::Global(EntityEvent::Created)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("Global created event received {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        if app.global::<AppState>().get_manifest_is_open() {
                            fill_project_tab(&app, &ctx);
                        }
                    }
                });
            }
        },
    );
}

/// Subscribe to Global update events to refresh ProjectTabState
fn subscribe_global_updated_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::Global(EntityEvent::Updated)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("Global updated event received {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        if app.global::<AppState>().get_manifest_is_open() {
                            app.global::<AppState>().set_manifest_is_saved(false);
                            fill_project_tab(&app, &ctx);
                        }
                    }
                });
            }
        },
    );
}

/// Helper function to get the global_id from root
fn get_global_id(app: &App, app_context: &Arc<AppContext>) -> Option<common::types::EntityId> {
    let workspace_id = app.global::<AppState>().get_workspace_id() as common::types::EntityId;
    if workspace_id > 0 {
        if let Ok(Some(workspace)) = workspace_commands::get_workspace(app_context, &workspace_id) {
            if workspace.global > 0 {
                println!("Found global_id: {}", workspace.global);
                return Some(workspace.global);
            }
        }
    }
    None
}

/// Helper function to update a global field with new value
fn update_global_helper<F>(app: &App, app_context: &Arc<AppContext>, update_fn: F)
where
    F: FnOnce(&mut direct_access::GlobalDto),
{
    if let Some(global_id) = get_global_id(app, app_context) {
        let global_res = global_commands::get_global(app_context, &global_id);

        if let Ok(Some(mut global)) = global_res {
            update_fn(&mut global);
            match global_commands::update_global(
                app_context,
                Some(app.global::<ProjectTabState>().get_project_undo_stack_id() as u64),
                &global,
            ) {
                Ok(_) => {
                    log::info!("Global updated successfully");
                }
                Err(e) => {
                    log::error!("Failed to update global: {}", e);
                }
            }
        }
    }
}

fn setup_language_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ProjectTabState>().on_language_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_value| {
            if let Some(app) = app_weak.upgrade() {
                let value_str = new_value.to_string();
                update_global_helper(&app, &ctx, |global| {
                    global.language = match value_str.as_str() {
                        "Rust" => "rust".to_string(),
                        "C++ / Qt" => "cpp-qt".to_string(),
                        _ => value_str.to_lowercase(),
                    };
                });
            }
        }
    });
}

fn setup_application_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ProjectTabState>()
        .on_application_name_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |new_value| {
                if let Some(app) = app_weak.upgrade() {
                    let value_str = new_value.to_string();
                    update_global_helper(&app, &ctx, |global| {
                        global.application_name = value_str;
                    });
                }
            }
        });
}

fn setup_organisation_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ProjectTabState>()
        .on_organisation_name_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |new_value| {
                if let Some(app) = app_weak.upgrade() {
                    let value_str = new_value.to_string();
                    update_global_helper(&app, &ctx, |global| {
                        global.organisation_name = value_str;
                    });
                }
            }
        });
}

fn setup_organisation_domain_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ProjectTabState>()
        .on_organisation_domain_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |new_value| {
                if let Some(app) = app_weak.upgrade() {
                    let value_str = new_value.to_string();
                    update_global_helper(&app, &ctx, |global| {
                        global.organisation_domain = value_str;
                    });
                }
            }
        });
}

fn setup_prefix_path_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ProjectTabState>().on_prefix_path_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_value| {
            if let Some(app) = app_weak.upgrade() {
                let value_str = new_value.to_string();
                update_global_helper(&app, &ctx, |global| {
                    global.prefix_path = value_str;
                });
            }
        }
    });
}

/// Initialize all project tab related callbacks
pub fn init(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    subscribe_global_created_event(event_hub_client, app, app_context);
    subscribe_global_updated_event(event_hub_client, app, app_context);
    subscribe_new_manifest_event(event_hub_client, app, app_context);
    subscribe_close_manifest_event(event_hub_client, app, app_context);
    subscribe_load_manifest_event(event_hub_client, app, app_context);
    setup_language_callback(app, app_context);
    setup_application_name_callback(app, app_context);
    setup_organisation_name_callback(app, app_context);
    setup_organisation_domain_callback(app, app_context);
    setup_prefix_path_callback(app, app_context);
}
