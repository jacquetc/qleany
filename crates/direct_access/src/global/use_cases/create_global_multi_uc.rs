use super::GlobalUnitOfWorkFactoryTrait;
use crate::global::dtos::{CreateGlobalDto, GlobalDto};
use anyhow::{Ok, Result};
use common::entities::Global;
use common::undo_redo::UndoRedoCommand;
use std::collections::VecDeque;

pub struct CreateGlobalMultiUseCase {
    uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Global>>,
    redo_stack: VecDeque<Vec<Global>>,
}

impl CreateGlobalMultiUseCase {
    pub fn new(uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>) -> Self {
        CreateGlobalMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }
    pub fn execute(&mut self, dtos: &[CreateGlobalDto]) -> Result<Vec<GlobalDto>> {
        // create
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let globals =
            uow.create_global_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        //store in undo stack
        self.undo_stack.push_back(globals.clone());
        self.redo_stack.clear();

        Ok(globals.into_iter().map(|global| global.into()).collect())
    }
}

impl UndoRedoCommand for CreateGlobalMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_globals) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_global_multi(
                &last_globals
                    .iter()
                    .map(|global| global.id.clone())
                    .collect::<Vec<_>>(),
            )?;
            uow.commit()?;
            self.redo_stack.push_back(last_globals);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_globals) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_global_multi(&last_globals)?;
            uow.commit()?;
            self.undo_stack.push_back(last_globals);
        }
        Ok(())
    }
}
