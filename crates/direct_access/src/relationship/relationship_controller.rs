use super::{
    dtos::{CreateRelationshipDto, RelationshipDto},
    units_of_work::{RelationshipUnitOfWorkFactory, RelationshipUnitOfWorkROFactory},
    use_cases::{
        create_relationship_multi_uc::CreateRelationshipMultiUseCase,
        create_relationship_uc::CreateRelationshipUseCase,
        get_relationship_multi_uc::GetRelationshipMultiUseCase,
        get_relationship_uc::GetRelationshipUseCase,
        remove_relationship_multi_uc::RemoveRelationshipMultiUseCase,
        remove_relationship_uc::RemoveRelationshipUseCase,
        update_relationship_multi_uc::UpdateRelationshipMultiUseCase,
        update_relationship_uc::UpdateRelationshipUseCase,
    },
};
use crate::RelationshipRelationshipDto;
use crate::relationship::use_cases::get_relationship_relationship_uc::GetRelationshipRelationshipUseCase;
use crate::relationship::use_cases::set_relationship_relationship_uc::SetRelationshipRelationshipUseCase;
use anyhow::{Ok, Result};
use common::direct_access::relationship::RelationshipRelationshipField;
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, event::EventHub, types::EntityId};
use std::sync::Arc;

pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    relationship: &CreateRelationshipDto,
) -> Result<RelationshipDto> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = CreateRelationshipUseCase::new(Box::new(uow_factory));
    let result = relationship_uc.execute(relationship.clone())?;
    undo_redo_manager.add_command_to_stack(Box::new(relationship_uc), stack_id)?;
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<RelationshipDto>> {
    let uow_factory = RelationshipUnitOfWorkROFactory::new(&db_context);
    let relationship_uc = GetRelationshipUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    relationship: &RelationshipDto,
) -> Result<RelationshipDto> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = UpdateRelationshipUseCase::new(Box::new(uow_factory));
    let result = relationship_uc.execute(relationship)?;
    undo_redo_manager.add_command_to_stack(Box::new(relationship_uc), stack_id)?;
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    id: &EntityId,
) -> Result<()> {
    // delete relationship
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = RemoveRelationshipUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(id)?;
    undo_redo_manager.add_command_to_stack(Box::new(relationship_uc), stack_id)?;
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    relationships: &[CreateRelationshipDto],
) -> Result<Vec<RelationshipDto>> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = CreateRelationshipMultiUseCase::new(Box::new(uow_factory));
    let result = relationship_uc.execute(relationships)?;
    undo_redo_manager.add_command_to_stack(Box::new(relationship_uc), stack_id)?;
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<RelationshipDto>>> {
    let uow_factory = RelationshipUnitOfWorkROFactory::new(&db_context);
    let relationship_uc = GetRelationshipMultiUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    relationships: &[RelationshipDto],
) -> Result<Vec<RelationshipDto>> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = UpdateRelationshipMultiUseCase::new(Box::new(uow_factory));
    let result = relationship_uc.execute(relationships)?;
    undo_redo_manager.add_command_to_stack(Box::new(relationship_uc), stack_id)?;
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = RemoveRelationshipMultiUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(ids)?;
    undo_redo_manager.add_command_to_stack(Box::new(relationship_uc), stack_id)?;
    Ok(())
}

pub fn get_relationship(
    db_context: &DbContext,
    id: &EntityId,
    field: &RelationshipRelationshipField,
) -> Result<Vec<EntityId>> {
    let uow_factory = RelationshipUnitOfWorkROFactory::new(&db_context);
    let relationship_uc = GetRelationshipRelationshipUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(id, field)
}

pub fn set_relationship(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    dto: &RelationshipRelationshipDto,
) -> Result<()> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = SetRelationshipRelationshipUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(dto)?;
    undo_redo_manager.add_command_to_stack(Box::new(relationship_uc), stack_id)?;
    Ok(())
}
