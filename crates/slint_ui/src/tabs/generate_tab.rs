//! Generate tab - Rust file generation UI logic
//!
//! This module handles the Generate tab functionality including:
//! - Listing rust files to be generated
//! - Filtering files by group
//! - Previewing generated code
//! - Starting/cancelling file generation

use std::collections::HashMap;
use std::sync::Arc;

use slint::{ComponentHandle, Model, SharedString, VecModel};

use crate::app_context::AppContext;
use crate::commands::rust_file_generation_commands;
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, GenerateCommands, ListItem};
use rust_file_generation::{GenerateRustCodeDto, GenerateRustFilesDto, ListRustFilesDto};
use slint::{Timer, TimerMode};

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

                groups
                    .entry(group_name)
                    .or_insert_with(Vec::new)
                    .push(file_data);
            }

            // Collect unique group names and sort them
            let mut group_names: Vec<String> = groups.keys().cloned().collect();
            group_names.sort();

            // Build group list items with "All" as first entry (only "All" is checked by default)
            let mut group_items: Vec<ListItem> = vec![ListItem {
                id: 0,
                text: SharedString::from("All"),
                subtitle: SharedString::default(),
                checked: true,
            }];

            // Add actual groups starting from id 1 (unchecked by default)
            for (idx, group_name) in group_names.iter().enumerate() {
                group_items.push(ListItem {
                    id: (idx + 1) as i32,
                    text: SharedString::from(group_name.as_str()),
                    subtitle: SharedString::default(),
                    checked: false,
                });
            }

            // Build file list items (all files initially, all checked since "All" is checked)
            let file_items: Vec<ListItem> = result
                .file_names
                .iter()
                .enumerate()
                .map(|(idx, file_name)| ListItem {
                    id: result.file_ids[idx] as i32,
                    text: SharedString::from(file_name.as_str()),
                    subtitle: SharedString::default(),
                    checked: true,
                })
                .collect();

            // Compute text prefixes and bolds for bold filename display
            let (text_prefixes, text_bolds): (Vec<SharedString>, Vec<SharedString>) = result
                .file_names
                .iter()
                .map(|file_name| {
                    if let Some(last_slash) = file_name.rfind('/') {
                        let prefix = &file_name[..=last_slash];
                        let bold = &file_name[last_slash + 1..];
                        (SharedString::from(prefix), SharedString::from(bold))
                    } else {
                        // No slash, entire name is bold
                        (
                            SharedString::default(),
                            SharedString::from(file_name.as_str()),
                        )
                    }
                })
                .unzip();

            // Update selected files count
            let selected_count = file_items.iter().filter(|f| f.checked).count() as i32;

            // Update UI
            let group_model = std::rc::Rc::new(VecModel::from(group_items));
            let file_model = std::rc::Rc::new(VecModel::from(file_items));

            app.global::<AppState>()
                .set_group_cr_list(group_model.into());
            app.global::<AppState>().set_file_cr_list(file_model.into());

            // Store group names in group_list for lookup (with "All" as first entry)
            let mut group_list: Vec<SharedString> = vec![SharedString::from("All")];
            group_list.extend(group_names.iter().map(|s| SharedString::from(s.as_str())));
            let group_list_model = std::rc::Rc::new(VecModel::from(group_list));
            app.global::<AppState>()
                .set_group_list(group_list_model.into());

            // Store file names in file_list for lookup
            let file_list: Vec<SharedString> = result
                .file_names
                .iter()
                .map(|s| SharedString::from(s.as_str()))
                .collect();
            let file_list_model = std::rc::Rc::new(VecModel::from(file_list));
            app.global::<AppState>()
                .set_file_list(file_list_model.into());

            // Set text prefixes and bolds for bold filename display
            let text_prefixes_model = std::rc::Rc::new(VecModel::from(text_prefixes));
            let text_bolds_model = std::rc::Rc::new(VecModel::from(text_bolds));
            app.global::<AppState>()
                .set_file_text_prefixes(text_prefixes_model.into());
            app.global::<AppState>()
                .set_file_text_bolds(text_bolds_model.into());

            // Set selected files count
            app.global::<AppState>()
                .set_selected_files_count(selected_count);

            // Reset selection and filter
            app.global::<AppState>().set_selected_group_index(-1);
            app.global::<AppState>().set_selected_file_index(-1);
            app.global::<AppState>()
                .set_code_preview(SharedString::from(""));
            app.global::<AppState>()
                .set_file_filter_text(SharedString::from(""));

            log::info!(
                "Loaded {} groups and {} files",
                groups.len(),
                result.file_names.len()
            );
        }
        Err(e) => {
            log::error!("Failed to list rust files: {}", e);
            app.global::<AppState>()
                .set_error_message(SharedString::from(e.as_str()));
        }
    }
}

/// Filter files by selected group
fn filter_files_by_group(app: &App, app_context: &Arc<AppContext>, group_index: i32) {
    if group_index < 0 {
        let was_saved = app.global::<AppState>().get_manifest_is_saved();
        // No group selected, show all files
        refresh_file_lists(app, app_context);

        // Re-apply manifest_is_saved after a short delay
        Timer::single_shot(std::time::Duration::from_millis(800), {
            let app_weak = app.as_weak();
            move || {
                if let Some(app) = app_weak.upgrade() {
                    if was_saved {
                        app.global::<AppState>().set_manifest_is_saved(true);
                        println!("Re-applied manifest_is_saved after refresh");
                    }
                }
            }
        });
        return;
    }

    // Get the selected group name
    let group_list = app.global::<AppState>().get_group_list();
    if group_index as usize >= group_list.row_count() {
        return;
    }

    let selected_group = group_list
        .row_data(group_index as usize)
        .unwrap_or_default();
    let selected_group_str = selected_group.to_string();

    // Get all files and filter by group
    let dto = ListRustFilesDto {
        only_list_already_existing: false,
    };

    match rust_file_generation_commands::list_rust_files(app_context, &dto) {
        Ok(result) => {
            // If "All" is selected (index 0), show all files
            let filtered_files: Vec<ListItem> = if selected_group_str == "All" {
                result
                    .file_names
                    .iter()
                    .enumerate()
                    .map(|(idx, file_name)| ListItem {
                        id: result.file_ids[idx] as i32,
                        text: SharedString::from(file_name.as_str()),
                        subtitle: SharedString::default(),
                        checked: true,
                    })
                    .collect()
            } else {
                // Filter by group using file_groups from DTO
                result
                    .file_names
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| result.file_groups[*idx] == selected_group_str)
                    .map(|(idx, file_name)| ListItem {
                        id: result.file_ids[idx] as i32,
                        text: SharedString::from(file_name.as_str()),
                        subtitle: SharedString::default(),
                        checked: true,
                    })
                    .collect()
            };

            let file_model = std::rc::Rc::new(VecModel::from(filtered_files));
            app.global::<AppState>().set_file_cr_list(file_model.into());

            // Reset file selection
            app.global::<AppState>().set_selected_file_index(-1);
            app.global::<AppState>()
                .set_code_preview(SharedString::from(""));
        }
        Err(e) => {
            log::error!("Failed to filter files: {}", e);
        }
    }
}

/// Load code preview for selected file
fn load_code_preview(app: &App, app_context: &Arc<AppContext>, file_id: i32) {
    if file_id < 0 {
        app.global::<AppState>()
            .set_code_preview(SharedString::from(""));
        return;
    }

    let dto = GenerateRustCodeDto {
        file_id: file_id as u64,
    };

    match rust_file_generation_commands::generate_rust_code(app_context, &dto) {
        Ok(result) => {
            app.global::<AppState>()
                .set_code_preview(SharedString::from(result.generated_code.as_str()));
            log::info!("Loaded code preview for file {}", file_id);
        }
        Err(e) => {
            log::error!("Failed to generate code preview: {}", e);
            app.global::<AppState>()
                .set_code_preview(SharedString::from(format!("Error: {}", e).as_str()));
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
                // Preserve manifest_is_saved state
                let was_saved = app.global::<AppState>().get_manifest_is_saved();

                refresh_file_lists(&app, &ctx);

                // Re-apply manifest_is_saved after a short delay
                Timer::single_shot(std::time::Duration::from_millis(800), move || {
                    if was_saved {
                        app.global::<AppState>().set_manifest_is_saved(true);
                        println!("Re-applied manifest_is_saved after refresh");
                    }
                });
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
                let root_path = ".".to_string();
                let prefix = if in_temp {
                    "temp_generated".to_string()
                } else {
                    String::new()
                };

                let dto = GenerateRustFilesDto {
                    file_ids,
                    root_path,
                    prefix,
                };

                // Set running state
                app.global::<AppState>().set_generate_is_running(true);
                app.global::<AppState>().set_generate_progress(0.0);
                app.global::<AppState>()
                    .set_generate_message(SharedString::from("Starting generation..."));

                match rust_file_generation_commands::generate_rust_files(&ctx, &dto) {
                    Ok(operation_id) => {
                        log::info!("Started generation with operation ID: {}", operation_id);
                        // Poll for completion in a timer
                        let ctx_clone = Arc::clone(&ctx);
                        let app_weak_clone = app.as_weak();
                        let op_id = operation_id.clone();

                        slint::Timer::single_shot(
                            std::time::Duration::from_millis(500),
                            move || {
                                poll_generation_result(app_weak_clone, ctx_clone, op_id);
                            },
                        );
                    }
                    Err(e) => {
                        log::error!("Failed to start generation: {}", e);
                        app.global::<AppState>().set_generate_is_running(false);
                        app.global::<AppState>()
                            .set_error_message(SharedString::from(e.as_str()));
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
                log::info!(
                    "Generation complete: {} files generated",
                    result.files.len()
                );
                app.global::<AppState>().set_generate_is_running(false);
                app.global::<AppState>().set_generate_progress(1.0);
                app.global::<AppState>()
                    .set_generate_message(SharedString::from(
                        format!("Generated {} files", result.files.len()).as_str(),
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
                    app.global::<AppState>()
                        .set_generate_progress(current_progress + 0.1);
                }

                slint::Timer::single_shot(std::time::Duration::from_millis(500), move || {
                    poll_generation_result(app_weak_clone, ctx_clone, op_id);
                });
            }
            Err(e) => {
                log::error!("Error checking generation result: {}", e);
                app.global::<AppState>().set_generate_is_running(false);
                app.global::<AppState>()
                    .set_error_message(SharedString::from(e.as_str()));
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
                app.global::<AppState>()
                    .set_generate_message(SharedString::from("Cancelled"));

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
                let was_saved = app.global::<AppState>().get_manifest_is_saved();

                filter_files_by_group(&app, &ctx, group_index);


                // Re-apply manifest_is_saved after a short delay
                Timer::single_shot(std::time::Duration::from_millis(800), move || {
                    if was_saved {
                        app.global::<AppState>().set_manifest_is_saved(true);
                        println!("Re-applied manifest_is_saved after refresh");
                    }
                });
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
                // Preserve manifest_is_saved state
                let was_saved = app.global::<AppState>().get_manifest_is_saved();

                // Refresh file lists
                refresh_file_lists(&app, &ctx);

                // Re-apply manifest_is_saved after a short delay
                Timer::single_shot(std::time::Duration::from_millis(800), move || {
                    if was_saved {
                        app.global::<AppState>().set_manifest_is_saved(true);
                        println!("Re-applied manifest_is_saved after refresh");
                    }
                });
            }
        }
    });
}

/// Setup the group_check_changed callback for exclusive group selection
fn setup_group_check_changed_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_group_check_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |group_id, checked| {
            log::info!("Group check changed: id={}, checked={}", group_id, checked);

            if let Some(app) = app_weak.upgrade() {
                let was_saved = app.global::<AppState>().get_manifest_is_saved();
                
                if checked {
                    // When a group is checked, uncheck all other groups and display only files in this group
                    let group_list = app.global::<AppState>().get_group_cr_list();
                    let group_names = app.global::<AppState>().get_group_list();

                    // Update group checkboxes: only the selected group is checked
                    let mut updated_groups: Vec<ListItem> = Vec::new();
                    for i in 0..group_list.row_count() {
                        if let Some(mut item) = group_list.row_data(i) {
                            item.checked = item.id == group_id;
                            updated_groups.push(item);
                        }
                    }
                    let group_model = std::rc::Rc::new(VecModel::from(updated_groups));
                    app.global::<AppState>()
                        .set_group_cr_list(group_model.into());

                    // Get the selected group name
                    let selected_group_name = if group_id == 0 {
                        "All".to_string()
                    } else if (group_id as usize) < group_names.row_count() {
                        group_names
                            .row_data(group_id as usize)
                            .unwrap_or_default()
                            .to_string()
                    } else {
                        return;
                    };

                    // Get all files and filter to show only files from the selected group (like clicking on the group)
                    let dto = ListRustFilesDto {
                        only_list_already_existing: false,
                    };

                    if let Ok(result) = rust_file_generation_commands::list_rust_files(&ctx, &dto) {
                        // Filter files by group (like clicking on the group item)
                        let filtered_indices: Vec<usize> = result
                            .file_names
                            .iter()
                            .enumerate()
                            .filter(|(idx, _)| {
                                selected_group_name == "All"
                                    || result.file_groups[*idx] == selected_group_name
                            })
                            .map(|(idx, _)| idx)
                            .collect();

                        let file_items: Vec<ListItem> = filtered_indices
                            .iter()
                            .map(|&idx| {
                                ListItem {
                                    id: result.file_ids[idx] as i32,
                                    text: SharedString::from(result.file_names[idx].as_str()),
                                    subtitle: SharedString::default(),
                                    checked: true, // All displayed files are checked
                                }
                            })
                            .collect();

                        // Compute text prefixes and bolds for filtered files
                        let (text_prefixes, text_bolds): (Vec<SharedString>, Vec<SharedString>) =
                            filtered_indices
                                .iter()
                                .map(|&idx| {
                                    let file_name = &result.file_names[idx];
                                    if let Some(last_slash) = file_name.rfind('/') {
                                        let prefix = &file_name[..=last_slash];
                                        let bold = &file_name[last_slash + 1..];
                                        (SharedString::from(prefix), SharedString::from(bold))
                                    } else {
                                        (
                                            SharedString::default(),
                                            SharedString::from(file_name.as_str()),
                                        )
                                    }
                                })
                                .unzip();

                        // Update selected files count
                        let selected_count = file_items.iter().filter(|f| f.checked).count() as i32;

                        let file_model = std::rc::Rc::new(VecModel::from(file_items));
                        app.global::<AppState>().set_file_cr_list(file_model.into());

                        let text_prefixes_model = std::rc::Rc::new(VecModel::from(text_prefixes));
                        let text_bolds_model = std::rc::Rc::new(VecModel::from(text_bolds));
                        app.global::<AppState>()
                            .set_file_text_prefixes(text_prefixes_model.into());
                        app.global::<AppState>()
                            .set_file_text_bolds(text_bolds_model.into());
                        app.global::<AppState>()
                            .set_selected_files_count(selected_count);

                        // Reset file selection and code preview
                        app.global::<AppState>().set_selected_file_index(-1);
                        app.global::<AppState>()
                            .set_code_preview(SharedString::from(""));


                        // Re-apply manifest_is_saved after a short delay
                        Timer::single_shot(std::time::Duration::from_millis(800), move || {
                            if was_saved {
                                app.global::<AppState>().set_manifest_is_saved(true);
                                println!("Re-applied manifest_is_saved after refresh");
                            }
                        });
                    }
                }
            }
        }
    });
}

/// Setup the file_filter_changed callback for filtering files by text
fn setup_file_filter_changed_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_file_filter_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |filter_text| {
            log::info!("File filter changed: {}", filter_text);
            if let Some(app) = app_weak.upgrade() {
                let filter_lower = filter_text.to_string().to_lowercase();

                // Get all files
                let dto = ListRustFilesDto {
                    only_list_already_existing: false,
                };

                if let Ok(result) = rust_file_generation_commands::list_rust_files(&ctx, &dto) {
                    // Get current group selection to maintain checked state
                    let group_list = app.global::<AppState>().get_group_cr_list();
                    let group_names = app.global::<AppState>().get_group_list();

                    // Find which group is checked
                    let mut checked_group_name = "All".to_string();
                    for i in 0..group_list.row_count() {
                        if let Some(item) = group_list.row_data(i) {
                            if item.checked {
                                if item.id == 0 {
                                    checked_group_name = "All".to_string();
                                } else if (item.id as usize) < group_names.row_count() {
                                    checked_group_name = group_names
                                        .row_data(item.id as usize)
                                        .unwrap_or_default()
                                        .to_string();
                                }
                                break;
                            }
                        }
                    }

                    // Filter files by text and build list
                    let filtered_indices: Vec<usize> = result
                        .file_names
                        .iter()
                        .enumerate()
                        .filter(|(_, file_name)| {
                            filter_lower.is_empty()
                                || file_name.to_lowercase().contains(&filter_lower)
                        })
                        .map(|(idx, _)| idx)
                        .collect();

                    let file_items: Vec<ListItem> = filtered_indices
                        .iter()
                        .map(|&idx| {
                            let file_name = &result.file_names[idx];
                            let file_group = &result.file_groups[idx];
                            let is_checked =
                                checked_group_name == "All" || file_group == &checked_group_name;
                            ListItem {
                                id: result.file_ids[idx] as i32,
                                text: SharedString::from(file_name.as_str()),
                                subtitle: SharedString::default(),
                                checked: is_checked,
                            }
                        })
                        .collect();

                    // Compute text prefixes and bolds for filtered files
                    let (text_prefixes, text_bolds): (Vec<SharedString>, Vec<SharedString>) =
                        filtered_indices
                            .iter()
                            .map(|&idx| {
                                let file_name = &result.file_names[idx];
                                if let Some(last_slash) = file_name.rfind('/') {
                                    let prefix = &file_name[..=last_slash];
                                    let bold = &file_name[last_slash + 1..];
                                    (SharedString::from(prefix), SharedString::from(bold))
                                } else {
                                    (
                                        SharedString::default(),
                                        SharedString::from(file_name.as_str()),
                                    )
                                }
                            })
                            .unzip();

                    // Update selected files count (only count checked files that are visible)
                    let selected_count = file_items.iter().filter(|f| f.checked).count() as i32;

                    let file_model = std::rc::Rc::new(VecModel::from(file_items));
                    app.global::<AppState>().set_file_cr_list(file_model.into());

                    let text_prefixes_model = std::rc::Rc::new(VecModel::from(text_prefixes));
                    let text_bolds_model = std::rc::Rc::new(VecModel::from(text_bolds));
                    app.global::<AppState>()
                        .set_file_text_prefixes(text_prefixes_model.into());
                    app.global::<AppState>()
                        .set_file_text_bolds(text_bolds_model.into());
                    app.global::<AppState>()
                        .set_selected_files_count(selected_count);
                }
            }
        }
    });
}

/// Setup the file_check_changed callback for updating selected files count
fn setup_file_check_changed_callback(app: &App, _app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_file_check_changed({
        let app_weak = app.as_weak();
        move |file_id, checked| {
            log::info!("File check changed: id={}, checked={}", file_id, checked);
            if let Some(app) = app_weak.upgrade() {
                // Update the checked state in the file list model
                let file_list = app.global::<AppState>().get_file_cr_list();
                let mut updated_files: Vec<ListItem> = Vec::new();

                for i in 0..file_list.row_count() {
                    if let Some(mut item) = file_list.row_data(i) {
                        if item.id == file_id {
                            item.checked = checked;
                        }
                        updated_files.push(item);
                    }
                }

                // Recalculate selected files count
                let selected_count = updated_files.iter().filter(|f| f.checked).count() as i32;

                let file_model = std::rc::Rc::new(VecModel::from(updated_files));
                app.global::<AppState>().set_file_cr_list(file_model.into());
                app.global::<AppState>()
                    .set_selected_files_count(selected_count);
            }
        }
    });
}

/// Setup the select_all_files callback
fn setup_select_all_files_callback(app: &App, _app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_select_all_files({
        let app_weak = app.as_weak();
        move || {
            log::info!("Select All Files clicked");
            if let Some(app) = app_weak.upgrade() {
                let file_list = app.global::<AppState>().get_file_cr_list();
                let mut updated_files: Vec<ListItem> = Vec::new();

                for i in 0..file_list.row_count() {
                    if let Some(mut item) = file_list.row_data(i) {
                        item.checked = true;
                        updated_files.push(item);
                    }
                }

                let selected_count = updated_files.len() as i32;
                let file_model = std::rc::Rc::new(VecModel::from(updated_files));
                app.global::<AppState>().set_file_cr_list(file_model.into());
                app.global::<AppState>()
                    .set_selected_files_count(selected_count);
            }
        }
    });
}

/// Setup the unselect_all_files callback
fn setup_unselect_all_files_callback(app: &App, _app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_unselect_all_files({
        let app_weak = app.as_weak();
        move || {
            log::info!("Unselect All Files clicked");
            if let Some(app) = app_weak.upgrade() {
                let file_list = app.global::<AppState>().get_file_cr_list();
                let mut updated_files: Vec<ListItem> = Vec::new();

                for i in 0..file_list.row_count() {
                    if let Some(mut item) = file_list.row_data(i) {
                        item.checked = false;
                        updated_files.push(item);
                    }
                }

                let file_model = std::rc::Rc::new(VecModel::from(updated_files));
                app.global::<AppState>().set_file_cr_list(file_model.into());
                app.global::<AppState>().set_selected_files_count(0);
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

    // Setup group check, file check, and file filter callbacks
    setup_group_check_changed_callback(app, app_context);
    setup_file_check_changed_callback(app, app_context);
    setup_file_filter_changed_callback(app, app_context);
    setup_select_all_files_callback(app, app_context);
    setup_unselect_all_files_callback(app, app_context);
}
