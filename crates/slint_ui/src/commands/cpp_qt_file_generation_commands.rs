//! c++/qt file generation commands for Slint UI

use crate::app_context::AppContext;
use common::long_operation::OperationProgress;
use cpp_qt_file_generation::{
    GenerateCppQtCodeDto, GenerateCppQtCodeReturnDto, GenerateCppQtFilesDto,
    GenerateCppQtFilesReturnDto, ListCppQtFilesDto, ListCppQtFilesReturnDto,
    cpp_qt_file_generation_controller,
};

/// List c++/qt files to be generated
pub fn list_cpp_qt_files(
    ctx: &AppContext,
    dto: &ListCppQtFilesDto,
) -> Result<ListCppQtFilesReturnDto, String> {
    cpp_qt_file_generation_controller::list_cpp_qt_files(&ctx.db_context, &ctx.event_hub, dto)
        .map_err(|e| format!("Error while listing c++/qt files: {:?}", e))
}

/// Generate c++/qt code (in memory)
pub fn generate_cpp_qt_code(
    ctx: &AppContext,
    dto: &GenerateCppQtCodeDto,
) -> Result<GenerateCppQtCodeReturnDto, String> {
    cpp_qt_file_generation_controller::generate_cpp_qt_code(&ctx.db_context, dto)
        .map_err(|e| format!("Error while generating c++/qt code: {:?}", e))
}

/// Start generating c++/qt files (long operation)
pub fn generate_cpp_qt_files(
    ctx: &AppContext,
    dto: &GenerateCppQtFilesDto,
) -> Result<String, String> {
    cpp_qt_file_generation_controller::generate_cpp_qt_files(
        &ctx.db_context,
        &ctx.event_hub,
        &mut ctx.long_operation_manager.lock().unwrap(),
        dto,
    )
    .map_err(|e| format!("Error while generating c++/qt files: {:?}", e))
}

/// Get the progress of a generate c++/qt files operation
pub fn get_generate_cpp_qt_files_progress(
    ctx: &AppContext,
    operation_id: &str,
) -> Result<Option<OperationProgress>, String> {
    Ok(
        cpp_qt_file_generation_controller::get_generate_cpp_qt_files_progress(
            &ctx.long_operation_manager.lock().unwrap(),
            operation_id,
        ),
    )
}

/// Get the result of a generate c++/qt files operation
pub fn get_generate_cpp_qt_files_result(
    ctx: &AppContext,
    operation_id: &str,
) -> Result<Option<GenerateCppQtFilesReturnDto>, String> {
    cpp_qt_file_generation_controller::get_generate_cpp_qt_files_result(
        &ctx.long_operation_manager.lock().unwrap(),
        operation_id,
    )
    .map_err(|e| {
        format!(
            "Error while getting c++/qt files generation result: {:?}",
            e
        )
    })
}
