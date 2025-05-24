use super::RootUnitOfWorkFactoryTrait;
use crate::root::dtos::{CreateRootDto, RootDto};
use anyhow::{Ok, Result};
use common::entities::Root;
use common::undo_redo::UndoRedoCommand;
use std::any::Any;
use std::collections::VecDeque;

pub struct CreateRootUseCase {
    uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Root>,
    redo_stack: VecDeque<Root>,
}

impl CreateRootUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>) -> Self {
        CreateRootUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: CreateRootDto) -> Result<RootDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let root = uow.create_root(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(root.clone());
        self.redo_stack.clear();

        Ok(root.into())
    }
}

impl UndoRedoCommand for CreateRootUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_root) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_root(&last_root.id)?;
            uow.commit()?;
            self.redo_stack.push_back(last_root);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_root) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_root(&last_root)?;
            uow.commit()?;
            self.undo_stack.push_back(last_root);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
