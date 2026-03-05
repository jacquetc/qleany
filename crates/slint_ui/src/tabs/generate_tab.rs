//! Generate tab - File generation UI logic
//!
//! This module handles the Generate tab functionality including:
//! - Listing files to be generated
//! - Computing file status (New, Modified, Unchanged) via fill_code + fill_status pipeline
//! - Filtering files by group, text, and status
//! - Previewing generated code
//! - Starting/cancelling file generation

use std::collections::BTreeSet;
use std::sync::Arc;

use slint::{ComponentHandle, Model, SharedString, VecModel};

use crate::app_context::AppContext;
use crate::commands::{
    cpp_qt_file_generation_commands, file_commands, file_generation_shared_steps_commands,
    global_commands, rust_file_generation_commands, system_commands,
};
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, GenerateCommands, ListItem};
use common::direct_access::system::SystemRelationshipField;
use common::entities::FileStatus;
use common::long_operation::OperationProgress;
use common::types::EntityId;
use cpp_qt_file_generation::{FillCppQtFilesDto, GenerateCppQtFilesDto};
use file_generation_shared_steps::GetDiffDto;
use rust_file_generation::{FillRustFilesDto, GenerateRustCodeDto, GenerateRustFilesDto};

const ROOT_SYSTEM_ID: u64 = 1;

enum Language {
    Rust,
    CppQt,
}

fn determine_language(app: &App, app_context: &Arc<AppContext>) -> Result<Language, String> {
    if let Some(global_id) = crate::tabs::common::get_global_id(app, app_context)
        && let Ok(Some(global)) = global_commands::get_global(app_context, &global_id)
    {
        match global.language.as_str() {
            "rust" => Ok(Language::Rust),
            "cpp-qt" => Ok(Language::CppQt),
            other => Err(format!("Unsupported language: {}", other)),
        }
    } else {
        Err("Failed to determine language".to_string())
    }
}

// ─── File display helpers ───────────────────────────────────────────────────

struct FileDisplayData {
    file_id: i32,
    display_name: String,
    group: String,
    status: FileStatus,
}

/// Read all files from DB and return display data
fn read_files_from_db(app_context: &Arc<AppContext>) -> Result<Vec<FileDisplayData>, String> {
    let file_ids = system_commands::get_system_relationship(
        app_context,
        &ROOT_SYSTEM_ID,
        &SystemRelationshipField::Files,
    )?;
    let files = file_commands::get_file_multi(app_context, &file_ids)?;

    Ok(files
        .into_iter()
        .flatten()
        .map(|f| FileDisplayData {
            file_id: f.id as i32,
            display_name: format!("{}{}", f.relative_path, f.name),
            group: f.group,
            status: f.status,
        })
        .collect())
}

/// Convert FileStatus to gradient color
fn status_to_color(status: &FileStatus) -> slint::Color {
    match status {
        FileStatus::Modified => slint::Color::from_rgb_u8(255, 152, 0), // orange
        FileStatus::New => slint::Color::from_rgb_u8(76, 175, 80),      // green
        FileStatus::Unchanged => slint::Color::from_rgb_u8(158, 158, 158), // grey
        FileStatus::Unknown => slint::Color::default(),                 // transparent
    }
}

/// Compute text display parts (prefix/bold split and elided versions) for file names
fn compute_display_parts(
    file_names: &[&str],
) -> (
    Vec<SharedString>, // text_prefixes
    Vec<SharedString>, // text_bolds
    Vec<SharedString>, // elided_texts
    Vec<SharedString>, // elided_prefixes
    Vec<SharedString>, // elided_bolds
) {
    let max_text_length = 50;
    let mut text_prefixes = Vec::new();
    let mut text_bolds = Vec::new();
    let mut elided_texts = Vec::new();
    let mut elided_prefixes = Vec::new();
    let mut elided_bolds = Vec::new();

    for file_name in file_names {
        // Split at last slash for bold display
        if let Some(last_slash) = file_name.rfind('/') {
            text_prefixes.push(SharedString::from(&file_name[..=last_slash]));
            text_bolds.push(SharedString::from(&file_name[last_slash + 1..]));
        } else {
            text_prefixes.push(SharedString::default());
            text_bolds.push(SharedString::from(*file_name));
        }

        // Elided versions for long names
        let char_count = file_name.chars().count();
        if char_count > max_text_length {
            let suffix: String = file_name
                .chars()
                .skip(char_count - max_text_length)
                .collect();
            let elided_full = format!("...{}", suffix);

            if let Some(last_slash) = elided_full.rfind('/') {
                elided_prefixes.push(SharedString::from(&elided_full[..=last_slash]));
                elided_bolds.push(SharedString::from(&elided_full[last_slash + 1..]));
            } else {
                elided_prefixes.push(SharedString::default());
                elided_bolds.push(SharedString::from(elided_full.as_str()));
            }
            elided_texts.push(SharedString::from(elided_full.as_str()));
        } else {
            elided_texts.push(SharedString::default());
            elided_prefixes.push(SharedString::default());
            elided_bolds.push(SharedString::default());
        }
    }

    (
        text_prefixes,
        text_bolds,
        elided_texts,
        elided_prefixes,
        elided_bolds,
    )
}

/// Set file UI models from filtered display data
fn set_file_ui_models(app: &App, filtered: &[&FileDisplayData]) {
    let file_items: Vec<ListItem> = filtered
        .iter()
        .map(|f| ListItem {
            id: f.file_id,
            text: SharedString::from(f.display_name.as_str()),
            subtitle: SharedString::default(),
            checked: true,
            gradient_color: status_to_color(&f.status),
        })
        .collect();

    let file_names: Vec<&str> = filtered.iter().map(|f| f.display_name.as_str()).collect();
    let (text_prefixes, text_bolds, elided_texts, elided_prefixes, elided_bolds) =
        compute_display_parts(&file_names);

    let selected_count = file_items.iter().filter(|f| f.checked).count() as i32;

    let state = app.global::<AppState>();
    state.set_file_cr_list(std::rc::Rc::new(VecModel::from(file_items)).into());
    state.set_file_text_prefixes(std::rc::Rc::new(VecModel::from(text_prefixes)).into());
    state.set_file_text_bolds(std::rc::Rc::new(VecModel::from(text_bolds)).into());
    state.set_file_elided_texts(std::rc::Rc::new(VecModel::from(elided_texts)).into());
    state.set_file_elided_prefixes(std::rc::Rc::new(VecModel::from(elided_prefixes)).into());
    state.set_file_elided_bolds(std::rc::Rc::new(VecModel::from(elided_bolds)).into());
    state.set_selected_files_count(selected_count);
    state.set_selected_file_index(-1);
    state.set_code_preview(SharedString::from(""));
}

/// Get the currently selected group name from UI state
fn get_selected_group_name(app: &App) -> String {
    let state = app.global::<AppState>();
    let group_list = state.get_group_list();
    let group_cr_list = state.get_group_cr_list();

    for i in 0..group_cr_list.row_count() {
        if let Some(item) = group_cr_list.row_data(i) {
            if item.checked {
                return group_list
                    .row_data(item.id as usize)
                    .unwrap_or_default()
                    .to_string();
            }
        }
    }
    "All".to_string()
}

/// Filter files by current UI state (group, text, show_all) and update display
fn update_file_display_from_db(app: &App, app_context: &Arc<AppContext>) {
    let files = match read_files_from_db(app_context) {
        Ok(f) => f,
        Err(e) => {
            log::error!("Failed to read files from DB: {}", e);
            return;
        }
    };

    let state = app.global::<AppState>();
    let show_all = state.get_show_all_files();
    let text_filter = state.get_file_filter_text().to_string().to_lowercase();
    let selected_group = get_selected_group_name(app);

    let filtered: Vec<&FileDisplayData> = files
        .iter()
        .filter(|f| {
            show_all
                || matches!(
                    f.status,
                    FileStatus::Modified | FileStatus::New | FileStatus::Unknown
                )
        })
        .filter(|f| selected_group == "All" || f.group == selected_group)
        .filter(|f| text_filter.is_empty() || f.display_name.to_lowercase().contains(&text_filter))
        .collect();

    set_file_ui_models(app, &filtered);
}

/// Build full display (groups + files) from DB
fn update_full_display(app: &App, app_context: &Arc<AppContext>) {
    let files = match read_files_from_db(app_context) {
        Ok(f) => f,
        Err(e) => {
            log::error!("Failed to read files from DB: {}", e);
            return;
        }
    };

    // Build sorted group list
    let mut group_set: BTreeSet<String> = BTreeSet::new();
    for f in &files {
        group_set.insert(f.group.clone());
    }
    let group_names: Vec<String> = group_set.into_iter().collect();

    // Build group items with "All" as first entry (checked by default)
    let mut group_items: Vec<ListItem> = vec![ListItem {
        id: 0,
        text: SharedString::from("All"),
        subtitle: SharedString::default(),
        checked: true,
        gradient_color: slint::Color::default(),
    }];
    for (idx, name) in group_names.iter().enumerate() {
        group_items.push(ListItem {
            id: (idx + 1) as i32,
            text: SharedString::from(name.as_str()),
            subtitle: SharedString::default(),
            checked: false,
            gradient_color: slint::Color::default(),
        });
    }

    let state = app.global::<AppState>();

    // Store group names for lookup
    let mut group_list: Vec<SharedString> = vec![SharedString::from("All")];
    group_list.extend(group_names.iter().map(|s| SharedString::from(s.as_str())));

    state.set_group_cr_list(std::rc::Rc::new(VecModel::from(group_items)).into());
    state.set_group_list(std::rc::Rc::new(VecModel::from(group_list)).into());

    // Reset selection and filter
    state.set_selected_group_index(-1);
    state.set_selected_file_index(-1);
    state.set_code_preview(SharedString::from(""));
    state.set_file_filter_text(SharedString::from(""));

    // Apply status filter and display files
    let show_all = state.get_show_all_files();
    let filtered: Vec<&FileDisplayData> = files
        .iter()
        .filter(|f| {
            show_all
                || matches!(
                    f.status,
                    FileStatus::Modified | FileStatus::New | FileStatus::Unknown
                )
        })
        .collect();

    set_file_ui_models(app, &filtered);

    log::info!(
        "Loaded {} groups and {} files ({} shown)",
        group_names.len(),
        files.len(),
        filtered.len()
    );
}

// ─── Fill code + status pipeline ────────────────────────────────────────────

/// Fill file entries in DB (step 1: quick)
fn fill_file_entries(app: &App, app_context: &Arc<AppContext>) -> Result<(), String> {
    match determine_language(app, app_context)? {
        Language::Rust => {
            let dto = FillRustFilesDto {
                only_list_already_existing: false,
            };
            rust_file_generation_commands::fill_rust_files(app_context, &dto).map(|_| ())
        }
        Language::CppQt => {
            let dto = FillCppQtFilesDto {
                only_list_already_existing: false,
            };
            cpp_qt_file_generation_commands::fill_cpp_qt_files(app_context, &dto).map(|_| ())
        }
    }
}

/// Start fill_code long operation (step 2: async)
fn start_fill_code(app: &App, app_context: &Arc<AppContext>) -> Result<String, String> {
    match determine_language(app, app_context)? {
        Language::Rust => rust_file_generation_commands::fill_code_in_rust_files(app_context),
        Language::CppQt => cpp_qt_file_generation_commands::fill_code_in_cpp_qt_files(app_context),
    }
}

/// Get fill_code progress
fn get_fill_code_progress(
    app: &App,
    app_context: &Arc<AppContext>,
    operation_id: &str,
) -> Result<Option<OperationProgress>, String> {
    match determine_language(app, app_context)? {
        Language::Rust => rust_file_generation_commands::get_fill_code_in_rust_files_progress(
            app_context,
            operation_id,
        ),
        Language::CppQt => cpp_qt_file_generation_commands::get_fill_code_in_cpp_qt_files_progress(
            app_context,
            operation_id,
        ),
    }
}

/// Get fill_code result
fn get_fill_code_result(
    app: &App,
    app_context: &Arc<AppContext>,
    operation_id: &str,
) -> Result<Option<()>, String> {
    match determine_language(app, app_context)? {
        Language::Rust => rust_file_generation_commands::get_fill_code_in_rust_files_result(
            app_context,
            operation_id,
        ),
        Language::CppQt => cpp_qt_file_generation_commands::get_fill_code_in_cpp_qt_files_result(
            app_context,
            operation_id,
        ),
    }
}

/// Poll fill_code operation, then fill status and update display when done
fn poll_fill_code_result(app_weak: slint::Weak<App>, ctx: Arc<AppContext>, operation_id: String) {
    if let Some(app) = app_weak.upgrade() {
        if !app.global::<AppState>().get_fill_code_is_running() {
            return;
        }

        // Update progress
        if let Ok(Some(progress)) = get_fill_code_progress(&app, &ctx, &operation_id) {
            app.global::<AppState>()
                .set_fill_code_progress(progress.percentage / 100.0);
            if let Some(msg) = progress.message {
                app.global::<AppState>()
                    .set_fill_code_message(SharedString::from(msg));
            }
        }

        // Check result
        match get_fill_code_result(&app, &ctx, &operation_id) {
            Ok(Some(())) => {
                // Fill code complete → fill status → update display
                log::info!("Fill code complete, computing status...");
                app.global::<AppState>()
                    .set_fill_code_message(SharedString::from("Comparing with disk..."));

                if let Err(e) = file_generation_shared_steps_commands::fill_status_in_files(&ctx) {
                    log::error!("Failed to fill status: {}", e);
                }

                update_full_display(&app, &ctx);
                app.global::<AppState>().set_fill_code_is_running(false);
            }
            Ok(None) => {
                // Still running, poll again
                let app_weak_clone = app.as_weak();
                let ctx_clone = Arc::clone(&ctx);
                let op_id = operation_id.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(200), move || {
                    poll_fill_code_result(app_weak_clone, ctx_clone, op_id);
                });
            }
            Err(e) => {
                log::error!("Error checking fill code result: {}", e);
                app.global::<AppState>().set_fill_code_is_running(false);
                // Show files without status
                update_full_display(&app, &ctx);
            }
        }
    }
}

/// Refresh file lists - runs the full pipeline (fill files → fill code → fill status → display)
fn refresh_file_lists(app: &App, app_context: &Arc<AppContext>) {
    // Step 1: Fill file entries in DB
    if let Err(e) = fill_file_entries(app, app_context) {
        log::error!("Failed to fill files: {}", e);
        app.global::<AppState>()
            .set_error_message(SharedString::from(e.as_str()));
        return;
    }

    // Step 2: Start fill_code long operation with progress
    app.global::<AppState>().set_fill_code_is_running(true);
    app.global::<AppState>().set_fill_code_progress(0.0);
    app.global::<AppState>()
        .set_fill_code_message(SharedString::from("Computing file status..."));

    match start_fill_code(app, app_context) {
        Ok(operation_id) => {
            poll_fill_code_result(app.as_weak(), Arc::clone(app_context), operation_id);
        }
        Err(e) => {
            log::error!("Failed to start fill code: {}", e);
            app.global::<AppState>().set_fill_code_is_running(false);
            // Show files without status
            update_full_display(app, app_context);
        }
    }
}

// ─── Code preview ───────────────────────────────────────────────────────────

/// Load code preview for selected file (generated code or diff depending on view_diff)
fn load_code_preview(app: &App, app_context: &Arc<AppContext>, file_id: i32) {
    if file_id < 0 {
        app.global::<AppState>()
            .set_code_preview(SharedString::from(""));
        return;
    }

    let view_diff = app.global::<AppState>().get_view_diff();

    if view_diff {
        load_diff_preview(app, app_context, file_id);
    } else {
        load_generated_code_preview(app, app_context, file_id);
    }
}

/// Load unified diff for the selected file
fn load_diff_preview(app: &App, app_context: &Arc<AppContext>, file_id: i32) {
    let dto = GetDiffDto {
        file_id: file_id as u64,
    };

    match file_generation_shared_steps_commands::get_file_diff(app_context, &dto) {
        Ok(result) => {
            let text = if result.diff_text.is_empty() {
                "No differences".to_string()
            } else {
                result.diff_text
            };
            app.global::<AppState>()
                .set_code_preview(SharedString::from(text.as_str()));
        }
        Err(e) => {
            log::error!("Failed to get file diff: {}", e);
            app.global::<AppState>()
                .set_code_preview(SharedString::from(format!("Error: {}", e).as_str()));
        }
    }
}

/// Load generated code preview for the selected file
fn load_generated_code_preview(app: &App, app_context: &Arc<AppContext>, file_id: i32) {
    match determine_language(app, app_context) {
        Ok(Language::Rust) => {
            let dto = GenerateRustCodeDto {
                file_id: file_id as u64,
            };

            match rust_file_generation_commands::generate_rust_code(app_context, &dto) {
                Ok(result) => {
                    app.global::<AppState>()
                        .set_code_preview(SharedString::from(result.generated_code.as_str()));
                }
                Err(e) => {
                    log::error!("Failed to generate code preview: {}", e);
                    app.global::<AppState>()
                        .set_code_preview(SharedString::from(format!("Error: {}", e).as_str()));
                }
            }
        }
        Ok(Language::CppQt) => {
            let dto = cpp_qt_file_generation::GenerateCppQtCodeDto {
                file_id: file_id as u64,
            };

            match cpp_qt_file_generation_commands::generate_cpp_qt_code(app_context, &dto) {
                Ok(result) => {
                    app.global::<AppState>()
                        .set_code_preview(SharedString::from(result.generated_code.as_str()));
                }
                Err(e) => {
                    log::error!("Failed to generate code preview: {}", e);
                    app.global::<AppState>()
                        .set_code_preview(SharedString::from(format!("Error: {}", e).as_str()));
                }
            }
        }
        Err(e) => {
            log::error!("Failed to determine language: {}", e);
            app.global::<AppState>()
                .set_code_preview(SharedString::from(format!("Error: {}", e).as_str()));
        }
    }
}

// ─── Callback setup ─────────────────────────────────────────────────────────

fn setup_list_files_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<GenerateCommands>().on_list_files({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("List Files clicked");
            if let Some(app) = app_weak.upgrade() {
                refresh_file_lists(&app, &ctx);
            }
        }
    });
}

fn setup_start_generate_callback(app: &App, app_context: &Arc<AppContext>) {
    fn generate_files_helper(
        app: &App,
        app_context: &Arc<AppContext>,
        file_ids: Vec<EntityId>,
        root_path: String,
        prefix: String,
    ) -> Result<String, String> {
        match determine_language(app, app_context)? {
            Language::Rust => {
                let dto = GenerateRustFilesDto {
                    file_ids,
                    root_path,
                    prefix,
                };

                rust_file_generation_commands::generate_rust_files(app_context, &dto)
            }
            Language::CppQt => {
                let dto = GenerateCppQtFilesDto {
                    file_ids,
                    root_path,
                    prefix,
                };

                cpp_qt_file_generation_commands::generate_cpp_qt_files(app_context, &dto)
            }
        }
    }

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
                    if let Some(item) = file_list.row_data(i)
                        && item.checked
                    {
                        file_ids.push(item.id as u64);
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
                    "temp".to_string()
                } else {
                    String::new()
                };

                // Set running state
                app.global::<AppState>().set_generate_is_running(true);
                app.global::<AppState>().set_generate_progress(0.0);
                app.global::<AppState>()
                    .set_generate_message(SharedString::from("Starting generation..."));

                match generate_files_helper(&app, &ctx, file_ids, root_path, prefix) {
                    Ok(operation_id) => {
                        log::info!("Started generation with operation ID: {}", operation_id);
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
    struct GenerateFilesReturnDto {
        pub files: Vec<String>,
        pub timestamp: String,
        pub duration: String,
    }

    fn get_generate_files_result_helper(
        app: &App,
        app_context: &Arc<AppContext>,
        operation_id: &str,
    ) -> Result<Option<GenerateFilesReturnDto>, String> {
        match determine_language(app, app_context)? {
            Language::Rust => {
                match rust_file_generation_commands::get_generate_rust_files_result(
                    app_context,
                    operation_id,
                ) {
                    Ok(Some(result)) => Ok(Some(GenerateFilesReturnDto {
                        files: result.files,
                        timestamp: result.timestamp,
                        duration: result.duration,
                    })),
                    Ok(None) => Ok(None),
                    Err(e) => Err(format!("Error getting rust generation result: {}", e)),
                }
            }
            Language::CppQt => {
                match cpp_qt_file_generation_commands::get_generate_cpp_qt_files_result(
                    app_context,
                    operation_id,
                ) {
                    Ok(Some(result)) => Ok(Some(GenerateFilesReturnDto {
                        files: result.files,
                        timestamp: result.timestamp,
                        duration: result.duration,
                    })),
                    Ok(None) => Ok(None),
                    Err(e) => Err(format!("Error getting C++/Qt generation result: {}", e)),
                }
            }
        }
    }
    fn get_generate_files_progress_helper(
        app: &App,
        app_context: &Arc<AppContext>,
        operation_id: &str,
    ) -> Result<Option<OperationProgress>, String> {
        match determine_language(app, app_context)? {
            Language::Rust => {
                match rust_file_generation_commands::get_generate_rust_files_progress(
                    app_context,
                    operation_id,
                ) {
                    Ok(Some(progress)) => Ok(Some(progress)),
                    Ok(None) => Ok(None),
                    Err(e) => Err(format!("Error getting Rust generation progress: {}", e)),
                }
            }
            Language::CppQt => {
                match cpp_qt_file_generation_commands::get_generate_cpp_qt_files_progress(
                    app_context,
                    operation_id,
                ) {
                    Ok(Some(progress)) => Ok(Some(progress)),
                    Ok(None) => Ok(None),
                    Err(e) => Err(format!("Error getting C++/Qt generation progress: {}", e)),
                }
            }
        }
    }

    if let Some(app) = app_weak.upgrade() {
        // Check if still running
        if !app.global::<AppState>().get_generate_is_running() {
            return; // Cancelled
        }

        match get_generate_files_result_helper(&app, &ctx, &operation_id) {
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

                // Update progress from backend
                if let Ok(Some(progress)) =
                    get_generate_files_progress_helper(&app, &ctx, &operation_id)
                {
                    app.global::<AppState>()
                        .set_generate_progress(progress.percentage / 100.0);
                    if let Some(msg) = progress.message {
                        app.global::<AppState>()
                            .set_generate_message(SharedString::from(msg));
                    }
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

fn setup_cancel_generate_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<GenerateCommands>().on_cancel_generate({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            log::info!("Cancel Generate clicked");
            if let Some(app) = app_weak.upgrade() {
                app.global::<AppState>().set_generate_is_running(false);
                app.global::<AppState>()
                    .set_generate_message(SharedString::from("Cancelled"));
                let _ = ctx;
            }
        }
    });
}

fn setup_group_selected_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_group_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |group_id| {
            log::info!("Group selected: id={}", group_id);

            if let Some(app) = app_weak.upgrade() {
                // Update group checkboxes: only the selected group is checked
                let group_list = app.global::<AppState>().get_group_cr_list();
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

                // Update selected group name for breadcrumb
                let group_names = app.global::<AppState>().get_group_list();
                if let Some(name) = group_names.row_data(group_id as usize) {
                    app.global::<AppState>().set_selected_group_name(name);
                }

                // Re-filter files from DB with new group selection
                update_file_display_from_db(&app, &ctx);
            }
        }
    });
}

fn setup_file_selected_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_file_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |file_id| {
            log::info!("File selected: {}", file_id);
            if let Some(app) = app_weak.upgrade() {
                // Find the file name by ID (not by index) for breadcrumb
                let file_list = app.global::<AppState>().get_file_cr_list();
                for i in 0..file_list.row_count() {
                    if let Some(item) = file_list.row_data(i) {
                        if item.id == file_id {
                            app.global::<AppState>().set_selected_file_name(item.text);
                            break;
                        }
                    }
                }

                load_code_preview(&app, &ctx, file_id);
            }
        }
    });
}

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

fn setup_file_filter_changed_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_file_filter_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |_filter_text| {
            if let Some(app) = app_weak.upgrade() {
                update_file_display_from_db(&app, &ctx);
            }
        }
    });
}

fn setup_file_check_changed_callback(app: &App, _app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_file_check_changed({
        let app_weak = app.as_weak();
        move |file_id, checked| {
            log::info!("File check changed: id={}, checked={}", file_id, checked);
            if let Some(app) = app_weak.upgrade() {
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

                let selected_count = updated_files.iter().filter(|f| f.checked).count() as i32;
                let file_model = std::rc::Rc::new(VecModel::from(updated_files));
                app.global::<AppState>().set_file_cr_list(file_model.into());
                app.global::<AppState>()
                    .set_selected_files_count(selected_count);
            }
        }
    });
}

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

fn setup_show_all_files_changed_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_show_all_files_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |_show_all| {
            if let Some(app) = app_weak.upgrade() {
                update_file_display_from_db(&app, &ctx);
            }
        }
    });
}

fn setup_view_diff_changed_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_view_diff_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |_view_diff| {
            if let Some(app) = app_weak.upgrade() {
                let file_id = app.global::<AppState>().get_selected_file_index();
                if file_id >= 0 {
                    load_code_preview(&app, &ctx, file_id);
                }
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

    // Setup filter callbacks
    setup_file_check_changed_callback(app, app_context);
    setup_file_filter_changed_callback(app, app_context);
    setup_select_all_files_callback(app, app_context);
    setup_unselect_all_files_callback(app, app_context);
    setup_show_all_files_changed_callback(app, app_context);
    setup_view_diff_changed_callback(app, app_context);
}
