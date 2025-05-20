use super::GlobalUnitOfWorkFactoryTrait;
use crate::global::dtos::GlobalDto;
use anyhow::{Ok, Result};
use common::{entities::Global, undo_redo::UndoRedoCommand};
use std::collections::VecDeque;

pub struct UpdateGlobalMultiUseCase {
    uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Global>>,
    redo_stack: VecDeque<Vec<Global>>,
}

impl UpdateGlobalMultiUseCase {
    pub fn new(uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>) -> Self {
        UpdateGlobalMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dtos: &[GlobalDto]) -> Result<Vec<GlobalDto>> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // check if id exists
        let mut exists = true;
        for GlobalDto { id, .. } in dtos {
            if uow.get_global(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }

        let globals =
            uow.update_global_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(globals.clone());
        self.redo_stack.clear();

        Ok(globals.into_iter().map(|global| global.into()).collect())
    }
}

impl UndoRedoCommand for UpdateGlobalMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_globals) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_global_multi(&last_globals)?;
            uow.commit()?;
            self.redo_stack.push_back(last_globals);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(globals) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_global_multi(&globals)?;
            uow.commit()?;
            self.undo_stack.push_back(globals);
        }
        Ok(())
    }
}
