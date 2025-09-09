use crate::units_of_work::generate_rust_code_uow::GenerateRustCodeUnitOfWorkFactory;
use crate::units_of_work::generate_rust_files_uow::GenerateRustFilesUnitOfWorkFactory;
use crate::use_cases::generate_rust_code_uc::GenerateRustCodeUseCase;
use crate::use_cases::generate_rust_files_uc::GenerateRustFilesUseCase;
use crate::{
    GenerateRustCodeDto, GenerateRustCodeReturnDto, GenerateRustFilesDto,
    GenerateRustFilesReturnDto, ListRustFilesDto, ListRustFilesReturnDto,
    units_of_work::list_files_uow::ListRustFilesUnitOfWorkFactory,
    use_cases::list_rust_files_uc::ListRustFilesUseCase,
};
use anyhow::Result;
use common::event::RustFileGenerationEvent::ListRustFiles;
use common::event::{Event, Origin};
use common::long_operation::LongOperationManager;
use common::{database::db_context::DbContext, event::EventHub};
use std::sync::Arc;

pub fn list_rust_files(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    dto: &ListRustFilesDto,
) -> Result<ListRustFilesReturnDto> {
    let uow_context = ListRustFilesUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut uc = ListRustFilesUseCase::new(Box::new(uow_context));
    let return_dto = uc.execute(dto)?;
    // Notify that the handling manifest has been loaded
    event_hub.send_event(Event {
        origin: Origin::RustFileGeneration(ListRustFiles),
        ids: vec![],
        data: None,
    });
    Ok(return_dto)
}

pub fn generate_rust_code(
    db_context: &DbContext,
    dto: &GenerateRustCodeDto,
) -> Result<GenerateRustCodeReturnDto> {
    let uow_context = GenerateRustCodeUnitOfWorkFactory::new(&db_context);
    let uc = GenerateRustCodeUseCase::new(Box::new(uow_context));
    let result = uc.execute(dto)?;
    Ok(result)
}

pub fn generate_rust_files(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    long_operation_manager: &mut LongOperationManager,
    dto: &GenerateRustFilesDto,
) -> Result<String> {
    let uow_context = GenerateRustFilesUnitOfWorkFactory::new(&db_context);
    let uc = GenerateRustFilesUseCase::new(Box::new(uow_context), dto);
    let operation_id = long_operation_manager.start_operation(uc);
    Ok(operation_id)
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

    // Parse the JSON string into a GenerateRustFilesResultDto
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
    fn test_list_rust_files() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let load_dto = ListRustFilesDto {
            only_list_already_existing: false,
        };
        list_rust_files(&db_context, &event_hub, &load_dto).unwrap();
    }
}
