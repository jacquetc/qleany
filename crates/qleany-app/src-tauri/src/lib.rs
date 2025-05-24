// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod direct_access_commands;
mod event_hub_client;
mod handling_manifest_commands;
use common::{database::db_context::DbContext, event::EventHub, undo_redo::UndoRedoManager};
use std::sync::Arc;
use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

#[derive(Clone)]
struct AppContext {
    pub db_context: DbContext,
    pub event_hub: Arc<EventHub>,
    pub event_hub_client: event_hub_client::EventHubClient,
    pub quit_signal: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub undo_redo_manager: Arc<Mutex<UndoRedoManager>>,
}

impl AppContext {
    fn new(app_handle: tauri::AppHandle) -> Self {
        let db_context = DbContext::new().unwrap();

        let event_hub = Arc::new(EventHub::new());
        let atomic_bool = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        event_hub.start_event_loop(atomic_bool.clone());
        let event_hub_client: event_hub_client::EventHubClient =
            event_hub_client::EventHubClient::new(&event_hub);
        event_hub_client.start(app_handle, atomic_bool.clone());

        Self {
            db_context,
            event_hub,
            event_hub_client,
            quit_signal: atomic_bool,
            undo_redo_manager: Arc::new(Mutex::new(UndoRedoManager::new())),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppContext::new(app.handle().clone())));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // root
            direct_access_commands::root_commands::get_root,
            direct_access_commands::root_commands::get_root_multi,
            direct_access_commands::root_commands::create_root,
            direct_access_commands::root_commands::create_root_multi,
            direct_access_commands::root_commands::update_root,
            direct_access_commands::root_commands::update_root_multi,
            direct_access_commands::root_commands::remove_root,
            direct_access_commands::root_commands::remove_root_multi,
            direct_access_commands::root_commands::get_root_relationship,
            direct_access_commands::root_commands::set_root_relationship,
            // entity
            direct_access_commands::entity_commands::get_entity,
            direct_access_commands::entity_commands::get_entity_multi,
            direct_access_commands::entity_commands::create_entity,
            direct_access_commands::entity_commands::create_entity_multi,
            direct_access_commands::entity_commands::update_entity,
            direct_access_commands::entity_commands::update_entity_multi,
            direct_access_commands::entity_commands::remove_entity,
            direct_access_commands::entity_commands::remove_entity_multi,
            direct_access_commands::entity_commands::get_entity_relationship,
            direct_access_commands::entity_commands::set_entity_relationship,
            // global
            direct_access_commands::global_commands::get_global,
            direct_access_commands::global_commands::get_global_multi,
            direct_access_commands::global_commands::create_global,
            direct_access_commands::global_commands::create_global_multi,
            direct_access_commands::global_commands::update_global,
            direct_access_commands::global_commands::update_global_multi,
            direct_access_commands::global_commands::remove_global,
            direct_access_commands::global_commands::remove_global_multi,
            // relationship
            direct_access_commands::relationship_commands::get_relationship,
            direct_access_commands::relationship_commands::get_relationship_multi,
            direct_access_commands::relationship_commands::create_relationship,
            direct_access_commands::relationship_commands::create_relationship_multi,
            direct_access_commands::relationship_commands::update_relationship,
            direct_access_commands::relationship_commands::update_relationship_multi,
            direct_access_commands::relationship_commands::remove_relationship,
            direct_access_commands::relationship_commands::remove_relationship_multi,
            direct_access_commands::relationship_commands::get_relationship_relationship,
            direct_access_commands::relationship_commands::set_relationship_relationship,
            // field
            direct_access_commands::field_commands::get_field,
            direct_access_commands::field_commands::get_field_multi,
            direct_access_commands::field_commands::create_field,
            direct_access_commands::field_commands::create_field_multi,
            direct_access_commands::field_commands::update_field,
            direct_access_commands::field_commands::update_field_multi,
            direct_access_commands::field_commands::remove_field,
            direct_access_commands::field_commands::remove_field_multi,
            direct_access_commands::field_commands::get_field_relationship,
            direct_access_commands::field_commands::set_field_relationship,
            // feature
            direct_access_commands::feature_commands::get_feature,
            direct_access_commands::feature_commands::get_feature_multi,
            direct_access_commands::feature_commands::create_feature,
            direct_access_commands::feature_commands::create_feature_multi,
            direct_access_commands::feature_commands::update_feature,
            direct_access_commands::feature_commands::update_feature_multi,
            direct_access_commands::feature_commands::remove_feature,
            direct_access_commands::feature_commands::remove_feature_multi,
            direct_access_commands::feature_commands::get_feature_relationship,
            direct_access_commands::feature_commands::set_feature_relationship,
            // use case
            direct_access_commands::use_case_commands::get_use_case,
            direct_access_commands::use_case_commands::get_use_case_multi,
            direct_access_commands::use_case_commands::create_use_case,
            direct_access_commands::use_case_commands::create_use_case_multi,
            direct_access_commands::use_case_commands::update_use_case,
            direct_access_commands::use_case_commands::update_use_case_multi,
            direct_access_commands::use_case_commands::remove_use_case,
            direct_access_commands::use_case_commands::remove_use_case_multi,
            direct_access_commands::use_case_commands::get_use_case_relationship,
            direct_access_commands::use_case_commands::set_use_case_relationship,
            // dto
            direct_access_commands::dto_commands::get_dto,
            direct_access_commands::dto_commands::get_dto_multi,
            direct_access_commands::dto_commands::create_dto,
            direct_access_commands::dto_commands::create_dto_multi,
            direct_access_commands::dto_commands::update_dto,
            direct_access_commands::dto_commands::update_dto_multi,
            direct_access_commands::dto_commands::remove_dto,
            direct_access_commands::dto_commands::remove_dto_multi,
            direct_access_commands::dto_commands::get_dto_relationship,
            direct_access_commands::dto_commands::set_dto_relationship,
            //dto field
            direct_access_commands::dto_field_commands::get_dto_field,
            direct_access_commands::dto_field_commands::get_dto_field_multi,
            direct_access_commands::dto_field_commands::create_dto_field,
            direct_access_commands::dto_field_commands::create_dto_field_multi,
            direct_access_commands::dto_field_commands::update_dto_field,
            direct_access_commands::dto_field_commands::update_dto_field_multi,
            direct_access_commands::dto_field_commands::remove_dto_field,
            direct_access_commands::dto_field_commands::remove_dto_field_multi,
            // handling manifest
            handling_manifest_commands::load_manifest,
            //handling_manifest_commands::save_manifest,
        ])
        .on_window_event(|app, event| {
            let app_context = app.state::<Mutex<AppContext>>();
            let app_context = app_context.blocking_lock();
            match event {
                tauri::WindowEvent::CloseRequested { .. } => {
                    app_context
                        .quit_signal
                        .store(true, std::sync::atomic::Ordering::Relaxed);
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
