use super::DtoFieldUnitOfWorkFactoryTrait;
use anyhow::{Ok, Result};
use common::types::Savepoint;
use common::{types::EntityId, undo_redo::UndoRedoCommand};
use std::collections::VecDeque;

pub struct RemoveDtoFieldUseCase {
    uow_factory: Box<dyn DtoFieldUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Savepoint>,
    redo_stack: VecDeque<EntityId>,
}

impl RemoveDtoFieldUseCase {
    pub fn new(uow_factory: Box<dyn DtoFieldUnitOfWorkFactoryTrait>) -> Self {
        RemoveDtoFieldUseCase {
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
        if uow.get_dto_field(&id)?.is_none() {
            return Err(anyhow::anyhow!("DtoField with id {} does not exist", id));
        }
        uow.delete_dto_field(id)?;
        uow.commit()?;

        // store savepoint in undo stack
        self.undo_stack.push_back(savepoint);
        self.redo_stack.push_back(id.clone());

        Ok(())
    }
}

impl UndoRedoCommand for RemoveDtoFieldUseCase {
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
            uow.delete_dto_field(&id)?;
            uow.commit()?;
            self.undo_stack.push_back(savepoint);
        }
        Ok(())
    }
}
