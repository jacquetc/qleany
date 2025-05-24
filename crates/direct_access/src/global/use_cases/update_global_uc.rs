use super::GlobalUnitOfWorkFactoryTrait;
use crate::global::dtos::GlobalDto;
use anyhow::{Ok, Result};
use common::{entities::Global, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateGlobalUseCase {
    uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Global>,
    redo_stack: VecDeque<Global>,
}

impl UpdateGlobalUseCase {
    pub fn new(uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>) -> Self {
        UpdateGlobalUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn description(&self) -> &str {
        "Update Global"
    }

    pub fn execute(&mut self, dto: &GlobalDto) -> Result<GlobalDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // check if id exists
        if uow.get_global(&dto.id)?.is_none() {
            return Err(anyhow::anyhow!("Global with id {} does not exist", dto.id));
        }

        // store in undo stack
        let global = uow.get_global(&dto.id)?.unwrap();
        self.undo_stack.push_back(global.clone());
        self.redo_stack.clear();

        let global = uow.update_global(&dto.into())?;
        uow.commit()?;

        Ok(global.into())
    }
}

impl UndoRedoCommand for UpdateGlobalUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_global) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_global(&last_global)?;
            uow.commit()?;
            self.redo_stack.push_back(last_global);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(global) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_global(&global)?;
            uow.commit()?;
            self.undo_stack.push_back(global);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
