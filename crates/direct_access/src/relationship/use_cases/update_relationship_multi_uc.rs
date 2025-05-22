use super::RelationshipUnitOfWorkFactoryTrait;
use crate::relationship::dtos::RelationshipDto;
use anyhow::{Ok, Result};
use common::{entities::Relationship, undo_redo::UndoRedoCommand};
use std::collections::VecDeque;

pub struct UpdateRelationshipMultiUseCase {
    uow_factory: Box<dyn RelationshipUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Relationship>>,
    redo_stack: VecDeque<Vec<Relationship>>,
}

impl UpdateRelationshipMultiUseCase {
    pub fn new(uow_factory: Box<dyn RelationshipUnitOfWorkFactoryTrait>) -> Self {
        UpdateRelationshipMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dtos: &[RelationshipDto]) -> Result<Vec<RelationshipDto>> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // check if id exists
        let mut exists = true;
        for RelationshipDto { id, .. } in dtos {
            if uow.get_relationship(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }

        let relationships =
            uow.update_relationship_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(relationships.clone());
        self.redo_stack.clear();

        Ok(relationships
            .into_iter()
            .map(|relationship| relationship.into())
            .collect())
    }
}

impl UndoRedoCommand for UpdateRelationshipMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_relationships) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_relationship_multi(&last_relationships)?;
            uow.commit()?;
            self.redo_stack.push_back(last_relationships);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(relationships) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_relationship_multi(&relationships)?;
            uow.commit()?;
            self.undo_stack.push_back(relationships);
        }
        Ok(())
    }
}
