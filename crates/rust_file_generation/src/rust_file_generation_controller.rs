use crate::{
    units_of_work::list_files_uow::ListRustFilesUnitOfWorkFactory,
    use_cases::list_rust_files_uc::ListRustFilesUseCase, ListRustFilesDto,
};
use anyhow::Result;
use common::event::RustFileGenerationEvent::ListFiles;
use common::event::{Event, Origin};
use common::{database::db_context::DbContext, event::EventHub};
use std::sync::Arc;

pub fn list_rust_files(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    dto: &ListRustFilesDto,
) -> Result<()> {
    let uow_context = ListRustFilesUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut uc = ListRustFilesUseCase::new(Box::new(uow_context));
    uc.execute(dto)?;
    // Notify that the handling manifest has been loaded
    event_hub.send_event(Event {
        origin: Origin::RustFileGeneration(ListFiles),
        ids: vec![],
        data: None,
    });
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_list_rust_files() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let load_dto = ListRustFilesDto {
            only_existing: false,
        };
        list_rust_files(&db_context, &event_hub, &load_dto).unwrap();
    }
}
