//! Generate tab - Rust file generation UI logic
//!
//! This module handles the Generate tab functionality including:
//! - Listing rust files to be generated
//! - Filtering files by group
//! - Previewing generated code
//! - Starting/cancelling file generation

use std::sync::Arc;
use std::collections::HashMap;

use slint::{ComponentHandle, Model, VecModel, SharedString};

use crate::app_context::AppContext;
use crate::commands::rust_file_generation_commands;
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, GenerateCommands, ListItem};
use rust_file_generation::{ListRustFilesDto, GenerateRustCodeDto, GenerateRustFilesDto};

/// Internal state for tracking file data
struct FileData {
    file_id: i32,
    file_name: String,
    group_name: String,
    checked: bool,
}

/// Refresh the file lists (groups and files)
fn refresh_file_lists(app: &App, app_context: &Arc<AppContext>) {
    let dto = ListRustFilesDto {
        only_list_already_existing: false,
    };

    match rust_file_generation_commands::list_rust_files(app_context, &dto) {
        Ok(result) => {
            // Build file data with groups from the DTO
            let mut groups: HashMap<String, Vec<FileData>> = HashMap::new();
            
            for (idx, file_name) in result.file_names.iter().enumerate() {
                let file_id = result.file_ids[idx] as i32;
                let group_name = result.file_groups[idx].clone();
                
                let file_data = FileData {
                    file_id,
                    file_name: file_name.clone(),
                    group_name: group_name.clone(),
                    checked: true,
                };
                
                groups.entry(group_name).or_insert_with(Vec::new).push(file_data);
            }
            
            // Collect unique group names and sort them
            let mut group_names: Vec<String> = groups.keys().cloned().collect();
            group_names.sort();
            
            // Build group list items with "All" as first entry
            let mut group_items: Vec<ListItem> = vec![
                ListItem {
                    id: 0,
                    text: SharedString::from("All"),
                    subtitle: SharedString::default(),
                    checked: true,
                }
            ];
            
            // Add actual groups starting from id 1
            for (idx, group_name) in group_names.iter().enumerate() {
                group_items.push(ListItem {
                    id: (idx + 1) as i32,
                    text: SharedString::from(group_name.as_str()),
                    subtitle: SharedString::default(),
                    checked: true,
                });
            }
            
            // Build file list items (all files initially)
            let file_items: Vec<ListItem> = result.file_names.iter().enumerate().map(|(idx, file_name)| {
                ListItem {
                    id: result.file_ids[idx] as i32,
                    text: SharedString::from(file_name.as_str()),
                    subtitle: SharedString::default(),
                    checked: true,
                }
            }).collect();
            
            // Update UI
            let group_model = std::rc::Rc::new(VecModel::from(group_items));
            let file_model = std::rc::Rc::new(VecModel::from(file_items));
            
            app.global::<AppState>().set_group_cr_list(group_model.into());
            app.global::<AppState>().set_file_cr_list(file_model.into());
            
            // Store group names in group_list for lookup (with "All" as first entry)
            let mut group_list: Vec<SharedString> = vec![SharedString::from("All")];
            group_list.extend(group_names.iter().map(|s| SharedString::from(s.as_str())));
            let group_list_model = std::rc::Rc::new(VecModel::from(group_list));
            app.global::<AppState>().set_group_list(group_list_model.into());
            
            // Store file names in file_list for lookup
            let file_list: Vec<SharedString> = result.file_names.iter().map(|s| SharedString::from(s.as_str())).collect();
            let file_list_model = std::rc::Rc::new(VecModel::from(file_list));
            app.global::<AppState>().set_file_list(file_list_model.into());
            
            // Reset selection
            app.global::<AppState>().set_selected_group_index(-1);
            app.global::<AppState>().set_selected_file_index(-1);
            app.global::<AppState>().set_code_preview(SharedString::from(""));
            
            log::info!("Loaded {} groups and {} files", groups.len(), result.file_names.len());
        }
        Err(e) => {
            log::error!("Failed to list rust files: {}", e);
            app.global::<AppState>().set_error_message(SharedString::from(e.as_str()));
        }
    }
}

/// Filter files by selected group
fn filter_files_by_group(app: &App, app_context: &Arc<AppContext>, group_index: i32) {
    if group_index < 0 {
        // No group selected, show all files
        refresh_file_lists(app, app_context);
        return;
    }
    
    // Get the selected group name
    let group_list = app.global::<AppState>().get_group_list();
    if group_index as usize >= group_list.row_count() {
        return;
    }
    
    let selected_group = group_list.row_data(group_index as usize).unwrap_or_default();
    let selected_group_str = selected_group.to_string();
    
    // Get all files and filter by group
    let dto = ListRustFilesDto {
        only_list_already_existing: false,
    };
    
    match rust_file_generation_commands::list_rust_files(app_context, &dto) {
        Ok(result) => {
            // If "All" is selected (index 0), show all files
            let filtered_files: Vec<ListItem> = if selected_group_str == "All" {
                result.file_names.iter().enumerate()
                    .map(|(idx, file_name)| {
                        ListItem {
                            id: result.file_ids[idx] as i32,
                            text: SharedString::from(file_name.as_str()),
                            subtitle: SharedString::default(),
                            checked: true,
                        }
                    })
                    .collect()
            } else {
                // Filter by group using file_groups from DTO
                result.file_names.iter().enumerate()
                    .filter(|(idx, _)| {
                        result.file_groups[*idx] == selected_group_str
                    })
                    .map(|(idx, file_name)| {
                        ListItem {
                            id: result.file_ids[idx] as i32,
                            text: SharedString::from(file_name.as_str()),
                            subtitle: SharedString::default(),
                            checked: true,
                        }
                    })
                    .collect()
            };
            
            let file_model = std::rc::Rc::new(VecModel::from(filtered_files));
            app.global::<AppState>().set_file_cr_list(file_model.into());
            
            // Reset file selection
            app.global::<AppState>().set_selected_file_index(-1);
            app.global::<AppState>().set_code_preview(SharedString::from(""));
        }
        Err(e) => {
            log::error!("Failed to filter files: {}", e);
        }
    }
}

/// Load code preview for selected file
fn load_code_preview(app: &App, app_context: &Arc<AppContext>, file_id: i32) {
    if file_id < 0 {
        app.global::<AppState>().set_code_preview(SharedString::from(""));
        return;
    }
    
    let dto = GenerateRustCodeDto {
        file_id: file_id as u64,
    };
    
    match rust_file_generation_commands::generate_rust_code(app_context, &dto) {
        Ok(result) => {
            app.global::<AppState>().set_code_preview(SharedString::from(result.generated_code.as_str()));
            log::info!("Loaded code preview for file {}", file_id);
        }
        Err(e) => {
            log::error!("Failed to generate code preview: {}", e);
            app.global::<AppState>().set_code_preview(SharedString::from(format!("Error: {}", e).as_str()));
        }
    }
}

/// Setup the list_files callback
fn setup_list_files_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<GenerateCommands>().on_list_files({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("List Rust Files clicked");
            if let Some(app) = app_weak.upgrade() {
                refresh_file_lists(&app, &ctx);
            }
        }
    });
}

/// Setup the start_generate callback
fn setup_start_generate_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<GenerateCommands>().on_start_generate({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("Start Generate clicked");
            if let Some(app) = app_weak.upgrade() {
                // Get checked file IDs from the file list
                let file_list = app.global::<AppState>().get_file_cr_list();
                let mut file_ids: Vec<u64> = Vec::new();
                
                for i in 0..file_list.row_count() {
                    if let Some(item) = file_list.row_data(i) {
                        if item.checked {
                            file_ids.push(item.id as u64);
                        }
                    }
                }
                
                if file_ids.is_empty() {
                    log::warn!("No files selected for generation");
                    return;
                }
                
                // Get generate_in_temp setting
                let in_temp = app.global::<AppState>().get_generate_in_temp();
                let root_path = if in_temp {
                    std::env::temp_dir().to_string_lossy().to_string()
                } else {
                    ".".to_string()
                };
                
                let dto = GenerateRustFilesDto {
                    file_ids,
                    root_path,
                    prefix: String::new(),
                };
                
                // Set running state
                app.global::<AppState>().set_generate_is_running(true);
                app.global::<AppState>().set_generate_progress(0.0);
                app.global::<AppState>().set_generate_message(SharedString::from("Starting generation..."));
                
                match rust_file_generation_commands::generate_rust_files(&ctx, &dto) {
                    Ok(operation_id) => {
                        log::info!("Started generation with operation ID: {}", operation_id);
                        // Poll for completion in a timer
                        let ctx_clone = Arc::clone(&ctx);
                        let app_weak_clone = app.as_weak();
                        let op_id = operation_id.clone();
                        
                        slint::Timer::single_shot(std::time::Duration::from_millis(500), move || {
                            poll_generation_result(app_weak_clone, ctx_clone, op_id);
                        });
                    }
                    Err(e) => {
                        log::error!("Failed to start generation: {}", e);
                        app.global::<AppState>().set_generate_is_running(false);
                        app.global::<AppState>().set_error_message(SharedString::from(e.as_str()));
                    }
                }
            }
        }
    });
}

/// Poll for generation result
fn poll_generation_result(app_weak: slint::Weak<App>, ctx: Arc<AppContext>, operation_id: String) {
    if let Some(app) = app_weak.upgrade() {
        // Check if still running
        if !app.global::<AppState>().get_generate_is_running() {
            return; // Cancelled
        }
        
        match rust_file_generation_commands::get_generate_rust_files_result(&ctx, &operation_id) {
            Ok(Some(result)) => {
                // Generation complete
                log::info!("Generation complete: {} files generated", result.files.len());
                app.global::<AppState>().set_generate_is_running(false);
                app.global::<AppState>().set_generate_progress(1.0);
                app.global::<AppState>().set_generate_message(SharedString::from(
                    format!("Generated {} files", result.files.len()).as_str()
                ));
            }
            Ok(None) => {
                // Still running, poll again
                let ctx_clone = Arc::clone(&ctx);
                let app_weak_clone = app.as_weak();
                let op_id = operation_id.clone();
                
                // Update progress (simulate)
                let current_progress = app.global::<AppState>().get_generate_progress();
                if current_progress < 0.9 {
                    app.global::<AppState>().set_generate_progress(current_progress + 0.1);
                }
                
                slint::Timer::single_shot(std::time::Duration::from_millis(500), move || {
                    poll_generation_result(app_weak_clone, ctx_clone, op_id);
                });
            }
            Err(e) => {
                log::error!("Error checking generation result: {}", e);
                app.global::<AppState>().set_generate_is_running(false);
                app.global::<AppState>().set_error_message(SharedString::from(e.as_str()));
            }
        }
    }
}

/// Setup the cancel_generate callback
fn setup_cancel_generate_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<GenerateCommands>().on_cancel_generate({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("Cancel Generate clicked");
            if let Some(app) = app_weak.upgrade() {
                // Cancel by setting running to false (the poll will stop)
                app.global::<AppState>().set_generate_is_running(false);
                app.global::<AppState>().set_generate_message(SharedString::from("Cancelled"));
                
                // TODO: Actually cancel the long operation via long_operation_manager
                let _ = ctx;
            }
        }
    });
}

/// Setup the group_selected callback
fn setup_group_selected_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_group_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |group_index| {
            log::info!("Group selected: {}", group_index);
            if let Some(app) = app_weak.upgrade() {
                filter_files_by_group(&app, &ctx, group_index);
            }
        }
    });
}

/// Setup the file_selected callback
fn setup_file_selected_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_file_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |file_id| {
            log::info!("File selected: {}", file_id);
            if let Some(app) = app_weak.upgrade() {
                load_code_preview(&app, &ctx, file_id);
            }
        }
    });
}

/// Setup the refresh_generate_tab callback
fn setup_refresh_generate_tab_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_refresh_generate_tab({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("Refreshing generate tab");
            if let Some(app) = app_weak.upgrade() {
                refresh_file_lists(&app, &ctx);
            }
        }
    });
}

/// Initialize all generate tab related subscriptions and callbacks
pub fn init(_event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    // Setup command callbacks
    setup_list_files_callback(app, app_context);
    setup_start_generate_callback(app, app_context);
    setup_cancel_generate_callback(app, app_context);
    
    // Setup selection callbacks
    setup_group_selected_callback(app, app_context);
    setup_file_selected_callback(app, app_context);
    setup_refresh_generate_tab_callback(app, app_context);
}
