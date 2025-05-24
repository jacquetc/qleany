use super::RootUnitOfWorkFactoryTrait;
use anyhow::{Ok, Result};
use common::types::Savepoint;
use common::{types::EntityId, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct RemoveRootUseCase {
    uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Savepoint>,
    redo_stack: VecDeque<EntityId>,
}

impl RemoveRootUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>) -> Self {
        RemoveRootUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let savepoint = uow.create_savepoint()?;
        // check if id exists
        if uow.get_root(&id)?.is_none() {
            return Err(anyhow::anyhow!("Root with id {} does not exist", id));
        }
        uow.delete_root(id)?;
        uow.commit()?;

        // store savepoint in undo stack
        self.undo_stack.push_back(savepoint);
        self.redo_stack.push_back(id.clone());

        Ok(())
    }
}

impl UndoRedoCommand for RemoveRootUseCase {
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
        if let Some(id) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            let savepoint = uow.create_savepoint()?;
            uow.delete_root(&id)?;
            uow.commit()?;
            self.undo_stack.push_back(savepoint);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
