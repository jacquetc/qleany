//! Qleany Slint UI Application
//! 
//! This application adapts the Tauri+React architecture to Slint:
//! - Clean separation of commands in dedicated modules
//! - Event passing from backend to UI via EventHubClient
//! - AppContext for shared state management
//! - Global singletons for UI state and commands

mod app_context;
mod commands;
mod event_hub_client;
mod tabs;

use std::sync::Arc;
use app_context::AppContext;
use common::event::{DirectAccessEntity, EntityEvent, HandlingManifestEvent, Origin};
use event_hub_client::EventHubClient;

slint::include_modules!();


fn main() {
    // Initialize logging
    env_logger::init();
    log::info!("Starting Qleany Slint UI");

    // Create the application context (backend state)
    let app_context = Arc::new(AppContext::new());
    
    // Create the event hub client for backend-to-UI event passing
    let event_hub_client = EventHubClient::new(&app_context.event_hub);
    event_hub_client.start(app_context.quit_signal.clone());

    // Create the Slint UI
    let app = App::new().unwrap();


    // Initialize global AppState (defaults are set in globals.slint, but we can override here if needed)
    // The globals are already initialized with defaults in globals.slint

    // Initialize entities tab subscriptions and callbacks
    tabs::entities_tab::init(&event_hub_client, &app, &app_context);

    // Initialize features tab subscriptions and callbacks
    tabs::features_tab::init(&event_hub_client, &app, &app_context);

    // Initialize home tab callbacks (manifest operations)
    tabs::home_tab::init(&event_hub_client, &app, &app_context);

    // Initialize project tab callbacks (project settings)
    tabs::project_tab::init(&event_hub_client, &app, &app_context);

    app.window().on_close_requested({
        let app_weak = app.as_weak();
        let ctx = Arc::clone(&app_context);

        move || {
            log::info!("Window close requested");

            let app = match app_weak.upgrade() {
                Some(app) => app,
                None => return slint::CloseRequestResponse::KeepWindowShown,
            };

            if app.global::<AppState>().get_manifest_is_saved() {
                log::info!("Manifest is saved, allowing window to close");
                ctx.shutdown();
                return slint::CloseRequestResponse::HideWindow;
            }
            app.global::<AppState>().set_confirm_dialog_pending_action(slint::SharedString::from("exit"));
            app.global::<AppState>().set_confirm_dialog_message(slint::SharedString::from("You have unsaved changes. Do you want to close the manifest without saving?"));
            app.global::<AppState>().set_confirm_dialog_visible(true);

            slint::CloseRequestResponse::KeepWindowShown
        }
    });

    app.global::<ManifestCommands>().on_exit_app({
        let ctx = Arc::clone(&app_context);
        move || {
            log::info!("Exit clicked");
            ctx.shutdown();
            std::process::exit(0);
        }
    });

    // Wire up UndoRedoCommands callbacks
    app.global::<UndoRedoCommands>().on_undo({
        let ctx = Arc::clone(&app_context);
        move || {
            log::info!("Undo clicked");
            // TODO: Implement undo using undo_redo_commands
            let _ = ctx;
        }
    });

    app.global::<UndoRedoCommands>().on_redo({
        let ctx = Arc::clone(&app_context);
        move || {
            log::info!("Redo clicked");
            // TODO: Implement redo using undo_redo_commands
            let _ = ctx;
        }
    });


    // Initialize generate tab callbacks (rust file generation)
    tabs::generate_tab::init(&event_hub_client, &app, &app_context);

    // Run the application
    log::info!("Running Slint UI");
    app.run().unwrap();

    // Cleanup on exit
    log::info!("Shutting down");
    app_context.shutdown();
}
