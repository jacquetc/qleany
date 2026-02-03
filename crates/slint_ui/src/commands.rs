//! Commands module - cleanly separated command handlers for Slint UI
//!
#![allow(dead_code)]

pub mod dto_commands;
pub mod dto_field_commands;
pub mod entity_commands;
pub mod feature_commands;
pub mod field_commands;
pub mod file_commands;
pub mod global_commands;
pub mod handling_manifest_commands;
pub mod relationship_commands;
pub mod system_commands;
pub mod user_interface_commands;
pub mod workspace_commands;

pub mod handling_app_lifecycle_commands;
pub mod rust_file_generation_commands;
pub mod cpp_qt_file_generation_commands;
pub mod undo_redo_commands;
pub mod use_case_commands;
