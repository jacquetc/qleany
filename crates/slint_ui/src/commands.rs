//! Commands module - cleanly separated command handlers for Slint UI
//! 
//! This module structure mirrors the Tauri implementation's command organization,
//! providing a clean separation of concerns for different entity types and operations.

pub mod handling_manifest_commands;
pub mod undo_redo_commands;
pub mod root_commands;
pub mod entity_commands;
pub mod feature_commands;
pub mod global_commands;
pub mod field_commands;
pub mod relationship_commands;
pub mod use_case_commands;
pub mod dto_commands;
pub mod dto_field_commands;
pub mod file_commands;
pub mod rust_file_generation_commands;
