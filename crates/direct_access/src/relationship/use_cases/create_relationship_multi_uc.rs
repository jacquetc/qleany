use super::RelationshipUnitOfWorkFactoryTrait;
use crate::relationship::dtos::{CreateRelationshipDto, RelationshipDto};
use anyhow::{Ok, Result};
use common::entities::Relationship;
use common::undo_redo::UndoRedoCommand;
use std::collections::VecDeque;

pub struct CreateRelationshipMultiUseCase {
    uow_factory: Box<dyn RelationshipUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Relationship>>,
    redo_stack: VecDeque<Vec<Relationship>>,
}

impl CreateRelationshipMultiUseCase {
    pub fn new(uow_factory: Box<dyn RelationshipUnitOfWorkFactoryTrait>) -> Self {
        CreateRelationshipMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }
    pub fn execute(&mut self, dtos: &[CreateRelationshipDto]) -> Result<Vec<RelationshipDto>> {
        // create
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let relationships =
            uow.create_relationship_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        //store in undo stack
        self.undo_stack.push_back(relationships.clone());
        self.redo_stack.clear();

        Ok(relationships
            .into_iter()
            .map(|relationship| relationship.into())
            .collect())
    }
}

impl UndoRedoCommand for CreateRelationshipMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_relationships) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_relationship_multi(
                &last_relationships
                    .iter()
                    .map(|relationship| relationship.id.clone())
                    .collect::<Vec<_>>(),
            )?;
            uow.commit()?;
            self.redo_stack.push_back(last_relationships);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_relationships) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_relationship_multi(&last_relationships)?;
            uow.commit()?;
            self.undo_stack.push_back(last_relationships);
        }
        Ok(())
    }
}
