use super::{
    dtos::{CreateFileDto, FileDto},
    units_of_work::{FileUnitOfWorkFactory, FileUnitOfWorkROFactory},
    use_cases::{
        create_file_multi_uc::CreateFileMultiUseCase, create_file_uc::CreateFileUseCase,
        get_file_multi_uc::GetFileMultiUseCase, get_file_uc::GetFileUseCase,
        remove_file_multi_uc::RemoveFileMultiUseCase, remove_file_uc::RemoveFileUseCase,
        update_file_multi_uc::UpdateFileMultiUseCase, update_file_uc::UpdateFileUseCase,
    },
};
use anyhow::{Ok, Result};
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, event::EventHub, types::EntityId};
use std::sync::Arc;

pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    file: &CreateFileDto,
) -> Result<FileDto> {
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = CreateFileUseCase::new(Box::new(uow_factory));
    let result = file_uc.execute(file.clone())?;
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<FileDto>> {
    let uow_factory = FileUnitOfWorkROFactory::new(&db_context);
    let file_uc = GetFileUseCase::new(Box::new(uow_factory));
    file_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    file: &FileDto,
) -> Result<FileDto> {
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = UpdateFileUseCase::new(Box::new(uow_factory));
    let result = file_uc.execute(file)?;
    Ok(result)
}

pub fn remove(db_context: &DbContext, event_hub: &Arc<EventHub>, id: &EntityId) -> Result<()> {
    // delete file
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = RemoveFileUseCase::new(Box::new(uow_factory));
    file_uc.execute(id)?;
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    files: &[CreateFileDto],
) -> Result<Vec<FileDto>> {
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = CreateFileMultiUseCase::new(Box::new(uow_factory));
    let result = file_uc.execute(files)?;
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<FileDto>>> {
    let uow_factory = FileUnitOfWorkROFactory::new(&db_context);
    let file_uc = GetFileMultiUseCase::new(Box::new(uow_factory));
    file_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    files: &[FileDto],
) -> Result<Vec<FileDto>> {
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = UpdateFileMultiUseCase::new(Box::new(uow_factory));
    let result = file_uc.execute(files)?;
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = RemoveFileMultiUseCase::new(Box::new(uow_factory));
    file_uc.execute(ids)?;
    Ok(())
}
