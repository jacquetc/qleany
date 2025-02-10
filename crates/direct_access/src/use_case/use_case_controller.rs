use super::{
    dtos::{CreateUseCaseDto, UseCaseDto},
    units_of_work::{UseCaseUnitOfWorkFactory, UseCaseUnitOfWorkROFactory},
    use_cases::{
        create_use_case_multi_uc::CreateUseCaseMultiUseCase,
        create_use_case_uc::CreateUseCaseUseCase, get_use_case_multi_uc::GetUseCaseMultiUseCase,
        get_use_case_uc::GetUseCaseUseCase, remove_use_case_multi_uc::RemoveUseCaseMultiUseCase,
        remove_use_case_uc::RemoveUseCaseUseCase,
        update_use_case_multi_uc::UpdateUseCaseMultiUseCase,
        update_use_case_uc::UpdateUseCaseUseCase,
    },
};
use anyhow::{Ok, Result};
use common::{database::db_context::DbContext, entities::EntityId, event::EventHub};
use common::undo_redo::UndoRedoManager;
use std::sync::Arc;
  
pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>, 
    undo_redo_manager: &mut UndoRedoManager,
    use_case: &CreateUseCaseDto,
) -> Result<UseCaseDto> {
    let uow_factory = UseCaseUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut use_case_uc = CreateUseCaseUseCase::new(Box::new(uow_factory));
    let result = use_case_uc.execute(use_case.clone())?;
    undo_redo_manager.add_command(Box::new(use_case_uc));
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<UseCaseDto>> {
    let uow_factory = UseCaseUnitOfWorkROFactory::new(&db_context);
    let use_case = GetUseCaseUseCase::new(Box::new(uow_factory));
    use_case.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    use_case: &UseCaseDto,
) -> Result<UseCaseDto> {
    let uow_factory = UseCaseUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut use_case_uc = UpdateUseCaseUseCase::new(Box::new(uow_factory));
    let result = use_case_uc.execute(use_case)?;
    undo_redo_manager.add_command(Box::new(use_case_uc));
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    id: &EntityId,
) -> Result<()> {
    // delete use case
    let uow_factory = UseCaseUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut use_case = RemoveUseCaseUseCase::new(Box::new(uow_factory));
    use_case.execute(id)?;
    undo_redo_manager.add_command(Box::new(use_case));
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    use_cases: &[CreateUseCaseDto],
) -> Result<Vec<UseCaseDto>> {
    let uow_factory = UseCaseUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut use_case_uc = CreateUseCaseMultiUseCase::new(Box::new(uow_factory));
    let result = use_case_uc.execute(use_cases)?;
    undo_redo_manager.add_command(Box::new(use_case_uc));
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<UseCaseDto>>> {
    let uow_factory = UseCaseUnitOfWorkROFactory::new(&db_context);
    let use_case_uc = GetUseCaseMultiUseCase::new(Box::new(uow_factory));
    use_case_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    use_cases: &[UseCaseDto],
) -> Result<Vec<UseCaseDto>> {
    let uow_factory = UseCaseUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut use_case_uc = UpdateUseCaseMultiUseCase::new(Box::new(uow_factory));
    let result = use_case_uc.execute(use_cases)?;
    undo_redo_manager.add_command(Box::new(use_case_uc));
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = UseCaseUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut use_case_uc = RemoveUseCaseMultiUseCase::new(Box::new(uow_factory));
    use_case_uc.execute(ids)?;
    undo_redo_manager.add_command(Box::new(use_case_uc));
    Ok(())
}
