use crate::units_of_work::generate_rust_code_uow::GenerateRustCodeUnitOfWorkFactory;
use crate::units_of_work::generate_rust_files_uow::GenerateRustFilesUnitOfWorkFactory;
use crate::use_cases::generate_rust_code_uc::GenerateRustCodeUseCase;
use crate::use_cases::generate_rust_files_uc::GenerateRustFilesUseCase;
use crate::{
    GenerateRustCodeDto, GenerateRustCodeReturnDto, GenerateRustFilesDto,
    GenerateRustFilesReturnDto, FillRustFilesDto, FillRustFilesReturnDto,
    units_of_work::list_files_uow::FillRustFilesUnitOfWorkFactory,
    use_cases::fill_rust_files_uc::FillRustFilesUseCase,
};
use anyhow::Result;
use common::event::RustFileGenerationEvent::FillRustFiles;
use common::event::{Event, Origin};
use common::long_operation::{LongOperationManager, OperationProgress};
use common::{database::db_context::DbContext, event::EventHub};
use std::sync::Arc;

pub fn fill_rust_files(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    dto: &FillRustFilesDto,
) -> Result<FillRustFilesReturnDto> {
    let uow_context = FillRustFilesUnitOfWorkFactory::new(db_context, event_hub);
    let mut uc = FillRustFilesUseCase::new(Box::new(uow_context));
    let return_dto = uc.execute(dto)?;
    // Notify that the handling manifest has been loaded
    event_hub.send_event(Event {
        origin: Origin::RustFileGeneration(FillRustFiles),
        ids: vec![],
        data: None,
    });
    Ok(return_dto)
}

pub fn generate_rust_code(
    db_context: &DbContext,
    dto: &GenerateRustCodeDto,
) -> Result<GenerateRustCodeReturnDto> {
    let uow_context = GenerateRustCodeUnitOfWorkFactory::new(db_context);
    let uc = GenerateRustCodeUseCase::new(Box::new(uow_context));
    let result = uc.execute(dto)?;
    Ok(result)
}

pub fn generate_rust_files(
    db_context: &DbContext,
    _event_hub: &Arc<EventHub>,
    long_operation_manager: &mut LongOperationManager,
    dto: &GenerateRustFilesDto,
) -> Result<String> {
    let uow_context = GenerateRustFilesUnitOfWorkFactory::new(db_context);
    let uc = GenerateRustFilesUseCase::new(Box::new(uow_context), dto);
    let operation_id = long_operation_manager.start_operation(uc);
    Ok(operation_id)
}

pub fn get_generate_rust_files_progress(
    long_operation_manager: &LongOperationManager,
    operation_id: &str,
) -> Option<OperationProgress> {
    long_operation_manager.get_operation_progress(operation_id)
}

pub fn get_generate_rust_files_result(
    long_operation_manager: &LongOperationManager,
    operation_id: &str,
) -> Result<Option<GenerateRustFilesReturnDto>> {
    // Get the operation result as a JSON string
    let result_json = long_operation_manager.get_operation_result(operation_id);

    // If there's no result, return None
    if result_json.is_none() {
        return Ok(None);
    }

    // Parse the JSON string into a GenerateRustFilesReturnDto
    let result_dto: GenerateRustFilesReturnDto = serde_json::from_str(&result_json.unwrap())?;

    Ok(Some(result_dto))
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    #[ignore]
    fn test_fill_rust_files() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let load_dto = FillRustFilesDto {
            only_list_already_existing: false,
        };
        fill_rust_files(&db_context, &event_hub, &load_dto).unwrap();
    }
}
