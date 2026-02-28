//! Rust file generation commands for Slint UI

use crate::app_context::AppContext;
use common::long_operation::OperationProgress;
use rust_file_generation::{
    GenerateRustCodeDto, GenerateRustCodeReturnDto, GenerateRustFilesDto,
    GenerateRustFilesReturnDto, FillRustFilesDto, FillRustFilesReturnDto,
    rust_file_generation_controller,
};

/// List rust files to be generated
pub fn fill_rust_files(
    ctx: &AppContext,
    dto: &FillRustFilesDto,
) -> Result<FillRustFilesReturnDto, String> {
    rust_file_generation_controller::fill_rust_files(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error while listing rust files: {:?}", e))
}

/// Generate rust code (in memory)
pub fn generate_rust_code(
    ctx: &AppContext,
    dto: &GenerateRustCodeDto,
) -> Result<GenerateRustCodeReturnDto, String> {
    rust_file_generation_controller::generate_rust_code(&ctx.db_context, dto)
        .map_err(|e| format!("Error while generating rust code: {:?}", e))
}

/// Start generating rust files (long operation)
pub fn generate_rust_files(ctx: &AppContext, dto: &GenerateRustFilesDto) -> Result<String, String> {
    rust_file_generation_controller::generate_rust_files(
        &ctx.db_context,
        &ctx.event_hub,
        &mut ctx.long_operation_manager.lock().unwrap(),
        dto,
    )
    .map_err(|e| format!("Error while generating rust files: {:?}", e))
}

/// Get the progress of a generate rust files operation
pub fn get_generate_rust_files_progress(
    ctx: &AppContext,
    operation_id: &str,
) -> Result<Option<OperationProgress>, String> {
    Ok(
        rust_file_generation_controller::get_generate_rust_files_progress(
            &ctx.long_operation_manager.lock().unwrap(),
            operation_id,
        ),
    )
}

/// Get the result of a generate rust files operation
pub fn get_generate_rust_files_result(
    ctx: &AppContext,
    operation_id: &str,
) -> Result<Option<GenerateRustFilesReturnDto>, String> {
    rust_file_generation_controller::get_generate_rust_files_result(
        &ctx.long_operation_manager.lock().unwrap(),
        operation_id,
    )
    .map_err(|e| format!("Error while getting rust files generation result: {:?}", e))
}
