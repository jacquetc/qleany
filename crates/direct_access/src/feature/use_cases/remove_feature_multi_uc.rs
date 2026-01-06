use super::FeatureUnitOfWorkFactoryTrait;
use anyhow::{Ok, Result};
use common::types::Savepoint;
use common::{types::EntityId, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct RemoveFeatureMultiUseCase {
    uow_factory: Box<dyn FeatureUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Savepoint>,
    redo_stack: VecDeque<Vec<EntityId>>,
}

impl RemoveFeatureMultiUseCase {
    pub fn new(uow_factory: Box<dyn FeatureUnitOfWorkFactoryTrait>) -> Self {
        RemoveFeatureMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, ids: &[EntityId]) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let savepoint = uow.create_savepoint()?;
        // check if id exists
        let mut exists = true;
        for id in ids {
            if uow.get_feature(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }
        uow.delete_feature_multi(ids)?;
        uow.commit()?;

        // store savepoint in undo stack
        self.undo_stack.push_back(savepoint);
        self.redo_stack.push_back(ids.to_vec());

        Ok(())
    }
}

impl UndoRedoCommand for RemoveFeatureMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(savepoint) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.restore_to_savepoint(savepoint)?;
            uow.commit()?;
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(ids) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            let savepoint = uow.create_savepoint()?;
            uow.delete_feature_multi(&ids)?;
            uow.commit()?;
            self.undo_stack.push_back(savepoint);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
