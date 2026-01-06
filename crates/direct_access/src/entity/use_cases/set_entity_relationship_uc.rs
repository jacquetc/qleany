use super::EntityUnitOfWorkFactoryTrait;
use crate::EntityRelationshipDto;
use anyhow::Result;
use common::types::{EntityId, Savepoint};
use common::undo_redo::UndoRedoCommand;
use std::any::Any;
use std::collections::VecDeque;

pub struct SetEntityRelationshipUseCase {
    uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<EntityRelationshipDto>,
    redo_stack: VecDeque<EntityRelationshipDto>,
}

impl SetEntityRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>) -> Self {
        SetEntityRelationshipUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: &EntityRelationshipDto) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // savepoint for undo
        let saved_relationship_ids = uow.get_entity_relationship(&dto.id, &dto.field)?;
        let undo_relationship = EntityRelationshipDto {
            id: dto.id.clone(),
            field: dto.field.clone(),
            right_ids: saved_relationship_ids,
        };
        //
        uow.set_entity_relationship(&dto.id, &dto.field, dto.right_ids.as_slice())?;
        uow.commit()?;
        // store savepoint in undo stack
        self.undo_stack.push_back(undo_relationship);
        self.redo_stack.push_back(dto.clone());

        Ok(())
    }
}

impl UndoRedoCommand for SetEntityRelationshipUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(undo_relationship) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.set_entity_relationship(
                &undo_relationship.id,
                &undo_relationship.field,
                &undo_relationship.right_ids,
            )?;
            uow.commit()?;
        }
        anyhow::Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(dto) = self.redo_stack.pop_back() {
            self.execute(&dto)?;
        }
        anyhow::Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
