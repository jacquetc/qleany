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
    tabs::home_tab::init(&app, &app_context);

    // Initialize project tab callbacks (project settings)
    tabs::project_tab::init(&app, &app_context);

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


    // Wire up GenerateCommands callbacks
    app.global::<GenerateCommands>().on_list_files({
        let ctx = Arc::clone(&app_context);
        move || {
            log::info!("List Rust Files clicked");
            // TODO: Implement using rust_file_generation crate
            let _ = ctx;
        }
    });

    app.global::<GenerateCommands>().on_start_generate({
        let ctx = Arc::clone(&app_context);
        move || {
            log::info!("Start Generate Rust Files clicked");
            // TODO: Implement using rust_file_generation crate and long_operation_manager
            let _ = ctx;
        }
    });

    app.global::<GenerateCommands>().on_cancel_generate({
        let ctx = Arc::clone(&app_context);
        move || {
            log::info!("Cancel Generate Rust Files clicked");
            // TODO: Implement cancellation using long_operation_manager
            let _ = ctx;
        }
    });


    event_hub_client.subscribe(
        Origin::HandlingManifest(HandlingManifestEvent::Loaded),
        {
            let ctx = Arc::clone(&app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("Manifest loaded event received: {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                // Use invoke_from_event_loop to safely update UI from background thread
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        app.global::<AppState>().set_manifest_is_open(true);
                        app.global::<AppState>().set_manifest_is_saved(true);
                    }
                });
            }
        },
    );

    // Run the application
    log::info!("Running Slint UI");
    app.run().unwrap();

    // Cleanup on exit
    log::info!("Shutting down");
    app_context.shutdown();
}
