use crate::units_of_work::generate_cpp_qt_code_uow::GenerateCppQtCodeUnitOfWorkFactory;
use crate::units_of_work::generate_cpp_qt_files_uow::GenerateCppQtFilesUnitOfWorkFactory;
use crate::use_cases::generate_cpp_qt_code_uc::GenerateCppQtCodeUseCase;
use crate::use_cases::generate_cpp_qt_files_uc::GenerateCppQtFilesUseCase;
use crate::{
    GenerateCppQtCodeDto, GenerateCppQtCodeReturnDto, GenerateCppQtFilesDto,
    GenerateCppQtFilesReturnDto, FillCppQtFilesDto, FillCppQtFilesReturnDto,
    units_of_work::list_files_uow::FillCppQtFilesUnitOfWorkFactory,
    use_cases::fill_cpp_qt_files_uc::FillCppQtFilesUseCase,
};
use anyhow::Result;
use common::event::CppQtFileGenerationEvent::FillCppQtFiles;
use common::event::{Event, Origin};
use common::long_operation::{LongOperationManager, OperationProgress};
use common::{database::db_context::DbContext, event::EventHub};
use std::sync::Arc;

pub fn fill_cpp_qt_files(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    dto: &FillCppQtFilesDto,
) -> Result<FillCppQtFilesReturnDto> {
    let uow_context = FillCppQtFilesUnitOfWorkFactory::new(db_context, event_hub);
    let mut uc = FillCppQtFilesUseCase::new(Box::new(uow_context));
    let return_dto = uc.execute(dto)?;
    // Notify that the handling manifest has been loaded
    event_hub.send_event(Event {
        origin: Origin::CppQtFileGeneration(FillCppQtFiles),
        ids: vec![],
        data: None,
    });
    Ok(return_dto)
}

pub fn generate_cpp_qt_code(
    db_context: &DbContext,
    dto: &GenerateCppQtCodeDto,
) -> Result<GenerateCppQtCodeReturnDto> {
    let uow_context = GenerateCppQtCodeUnitOfWorkFactory::new(db_context);
    let uc = GenerateCppQtCodeUseCase::new(Box::new(uow_context));
    let result = uc.execute(dto)?;
    Ok(result)
}

pub fn generate_cpp_qt_files(
    db_context: &DbContext,
    _event_hub: &Arc<EventHub>,
    long_operation_manager: &mut LongOperationManager,
    dto: &GenerateCppQtFilesDto,
) -> Result<String> {
    let uow_context = GenerateCppQtFilesUnitOfWorkFactory::new(db_context);
    let uc = GenerateCppQtFilesUseCase::new(Box::new(uow_context), dto);
    let operation_id = long_operation_manager.start_operation(uc);
    Ok(operation_id)
}

pub fn get_generate_cpp_qt_files_progress(
    long_operation_manager: &LongOperationManager,
    operation_id: &str,
) -> Option<OperationProgress> {
    long_operation_manager.get_operation_progress(operation_id)
}

pub fn get_generate_cpp_qt_files_result(
    long_operation_manager: &LongOperationManager,
    operation_id: &str,
) -> Result<Option<GenerateCppQtFilesReturnDto>> {
    // Get the operation result as a JSON string
    let result_json = long_operation_manager.get_operation_result(operation_id);

    // If there's no result, return None
    if result_json.is_none() {
        return Ok(None);
    }

    // Parse the JSON string into a GenerateCppQtFilesReturnDto
    let result_dto: GenerateCppQtFilesReturnDto = serde_json::from_str(&result_json.unwrap())?;

    Ok(Some(result_dto))
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    #[ignore]
    fn test_fill_cpp_qt_files() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let load_dto = FillCppQtFilesDto {
            only_list_already_existing: false,
        };
        fill_cpp_qt_files(&db_context, &event_hub, &load_dto).unwrap();
    }
}
