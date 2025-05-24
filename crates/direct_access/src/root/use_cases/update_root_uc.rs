use super::RootUnitOfWorkFactoryTrait;
use crate::root::dtos::RootDto;
use anyhow::{Ok, Result};
use common::{entities::Root, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateRootUseCase {
    uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Root>,
    redo_stack: VecDeque<Root>,
}

impl UpdateRootUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>) -> Self {
        UpdateRootUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn description(&self) -> &str {
        "Update Root"
    }

    pub fn execute(&mut self, dto: &RootDto) -> Result<RootDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // check if id exists
        if uow.get_root(&dto.id)?.is_none() {
            return Err(anyhow::anyhow!("Root with id {} does not exist", dto.id));
        }

        let root = uow.update_root(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(root.clone());
        self.redo_stack.clear();

        Ok(root.into())
    }
}

impl UndoRedoCommand for UpdateRootUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_root) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_root(&last_root)?;
            uow.commit()?;
            self.redo_stack.push_back(last_root);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(root) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_root(&root)?;
            uow.commit()?;
            self.undo_stack.push_back(root);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
