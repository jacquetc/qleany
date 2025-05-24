use super::RelationshipUnitOfWorkFactoryTrait;
use crate::relationship::dtos::RelationshipDto;
use anyhow::{Ok, Result};
use common::{entities::Relationship, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateRelationshipUseCase {
    uow_factory: Box<dyn RelationshipUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Relationship>,
    redo_stack: VecDeque<Relationship>,
}

impl UpdateRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn RelationshipUnitOfWorkFactoryTrait>) -> Self {
        UpdateRelationshipUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn description(&self) -> &str {
        "Update Relationship"
    }

    pub fn execute(&mut self, dto: &RelationshipDto) -> Result<RelationshipDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // check if id exists
        if uow.get_relationship(&dto.id)?.is_none() {
            return Err(anyhow::anyhow!(
                "Relationship with id {} does not exist",
                dto.id
            ));
        }

        let relationship = uow.update_relationship(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(relationship.clone());
        self.redo_stack.clear();

        Ok(relationship.into())
    }
}

impl UndoRedoCommand for UpdateRelationshipUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_relationship) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_relationship(&last_relationship)?;
            uow.commit()?;
            self.redo_stack.push_back(last_relationship);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(relationship) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_relationship(&relationship)?;
            uow.commit()?;
            self.undo_stack.push_back(relationship);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
