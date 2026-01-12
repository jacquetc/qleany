use super::{
    dtos::{CreateDtoDto, DtoDto},
    units_of_work::{DtoUnitOfWorkFactory, DtoUnitOfWorkROFactory},
    use_cases::{
        create_dto_multi_uc::CreateDtoMultiUseCase, create_dto_uc::CreateDtoUseCase,
        get_dto_multi_uc::GetDtoMultiUseCase, get_dto_uc::GetDtoUseCase,
        remove_dto_multi_uc::RemoveDtoMultiUseCase, remove_dto_uc::RemoveDtoUseCase,
        update_dto_multi_uc::UpdateDtoMultiUseCase, update_dto_uc::UpdateDtoUseCase,
    },
};
use crate::dto::use_cases::get_dto_relationship_uc::GetDtoRelationshipUseCase;
use crate::dto::use_cases::set_dto_relationship_uc::SetDtoRelationshipUseCase;
use crate::DtoRelationshipDto;
use anyhow::{Ok, Result};
use common::direct_access::dto::DtoRelationshipField;
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, event::EventHub, types::EntityId};
use std::sync::Arc;

pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    dto: &CreateDtoDto,
) -> Result<DtoDto> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = CreateDtoUseCase::new(Box::new(uow_factory));
    let result = dto_uc.execute(dto.clone())?;
    undo_redo_manager.add_command_to_stack(Box::new(dto_uc), stack_id)?;
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<DtoDto>> {
    let uow_factory = DtoUnitOfWorkROFactory::new(&db_context);
    let dto_uc = GetDtoUseCase::new(Box::new(uow_factory));
    dto_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    dto: &DtoDto,
) -> Result<DtoDto> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = UpdateDtoUseCase::new(Box::new(uow_factory));
    let result = dto_uc.execute(dto)?;
    undo_redo_manager.add_command_to_stack(Box::new(dto_uc), stack_id)?;
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    id: &EntityId,
) -> Result<()> {
    // delete dto
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = RemoveDtoUseCase::new(Box::new(uow_factory));
    dto_uc.execute(id)?;
    undo_redo_manager.add_command_to_stack(Box::new(dto_uc), stack_id)?;
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    dtos: &[CreateDtoDto],
) -> Result<Vec<DtoDto>> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = CreateDtoMultiUseCase::new(Box::new(uow_factory));
    let result = dto_uc.execute(dtos)?;
    undo_redo_manager.add_command_to_stack(Box::new(dto_uc), stack_id)?;
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<DtoDto>>> {
    let uow_factory = DtoUnitOfWorkROFactory::new(&db_context);
    let dto_uc = GetDtoMultiUseCase::new(Box::new(uow_factory));
    dto_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    dtos: &[DtoDto],
) -> Result<Vec<DtoDto>> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = UpdateDtoMultiUseCase::new(Box::new(uow_factory));
    let result = dto_uc.execute(dtos)?;
    undo_redo_manager.add_command_to_stack(Box::new(dto_uc), stack_id)?;
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = RemoveDtoMultiUseCase::new(Box::new(uow_factory));
    dto_uc.execute(ids)?;
    undo_redo_manager.add_command_to_stack(Box::new(dto_uc), stack_id)?;
    Ok(())
}

pub fn get_relationship(
    db_context: &DbContext,
    id: &EntityId,
    field: &DtoRelationshipField,
) -> Result<Vec<EntityId>> {
    let uow_factory = DtoUnitOfWorkROFactory::new(&db_context);
    let dto_uc = GetDtoRelationshipUseCase::new(Box::new(uow_factory));
    dto_uc.execute(id, field)
}

pub fn set_relationship(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    dto: &DtoRelationshipDto,
) -> Result<()> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = SetDtoRelationshipUseCase::new(Box::new(uow_factory));
    dto_uc.execute(dto)?;
    undo_redo_manager.add_command_to_stack(Box::new(dto_uc), stack_id)?;
    Ok(())
}
