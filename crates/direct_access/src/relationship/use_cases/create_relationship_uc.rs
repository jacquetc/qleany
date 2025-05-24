use super::RelationshipUnitOfWorkFactoryTrait;
use crate::relationship::dtos::{CreateRelationshipDto, RelationshipDto};
use anyhow::{Ok, Result};
use common::entities::Relationship;
use common::undo_redo::UndoRedoCommand;
use std::any::Any;
use std::collections::VecDeque;

pub struct CreateRelationshipUseCase {
    uow_factory: Box<dyn RelationshipUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Relationship>,
    redo_stack: VecDeque<Relationship>,
}

impl CreateRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn RelationshipUnitOfWorkFactoryTrait>) -> Self {
        CreateRelationshipUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: CreateRelationshipDto) -> Result<RelationshipDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let relationship = uow.create_relationship(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(relationship.clone());
        self.redo_stack.clear();

        Ok(relationship.into())
    }
}

impl UndoRedoCommand for CreateRelationshipUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_relationship) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_relationship(&last_relationship.id)?;
            uow.commit()?;
            self.redo_stack.push_back(last_relationship);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_relationship) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_relationship(&last_relationship)?;
            uow.commit()?;
            self.undo_stack.push_back(last_relationship);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
