//! Project Tab module
//!
//! This module contains the logic specific to the Project tab,
//! including callback handlers for project settings management.

use std::sync::Arc;

use slint::ComponentHandle;

use crate::app_context::AppContext;
use crate::commands::{global_commands, root_commands};
use crate::{App, ProjectTabState, AppState};

/// Helper function to get the global_id from root
fn get_global_id(app: &App, app_context: &Arc<AppContext>) -> Option<common::types::EntityId> {
    let root_id = app.global::<AppState>().get_root_id() as common::types::EntityId;
    if root_id > 0 {
        if let Ok(Some(root)) = root_commands::get_root(app_context, &root_id) {
            if root.global > 0 {
                return Some(root.global);
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
            match global_commands::update_global(app_context, &global) {
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
                    global.language = value_str;
                });
            }
        }
    });
}

fn setup_application_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<ProjectTabState>().on_application_name_changed({
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
    app.global::<ProjectTabState>().on_organisation_name_changed({
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
    app.global::<ProjectTabState>().on_organisation_domain_changed({
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
pub fn init(app: &App, app_context: &Arc<AppContext>) {
    setup_language_callback(app, app_context);
    setup_application_name_callback(app, app_context);
    setup_organisation_name_callback(app, app_context);
    setup_organisation_domain_callback(app, app_context);
    setup_prefix_path_callback(app, app_context);
}
