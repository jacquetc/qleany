use super::GlobalUnitOfWorkFactoryTrait;
use crate::global::dtos::{CreateGlobalDto, GlobalDto};
use anyhow::{Ok, Result};
use common::entities::Global;
use common::undo_redo::UndoRedoCommand;
use std::collections::VecDeque;

pub struct CreateGlobalUseCase {
    uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Global>,
    redo_stack: VecDeque<Global>,
}

impl CreateGlobalUseCase {
    pub fn new(uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>) -> Self {
        CreateGlobalUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: CreateGlobalDto) -> Result<GlobalDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let global = uow.create_global(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(global.clone());
        self.redo_stack.clear();

        Ok(global.into())
    }
}

impl UndoRedoCommand for CreateGlobalUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_global) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_global(&last_global.id)?;
            uow.commit()?;
            self.redo_stack.push_back(last_global);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_global) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_global(&last_global)?;
            uow.commit()?;
            self.undo_stack.push_back(last_global);
        }
        Ok(())
    }
}
