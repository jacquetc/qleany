use super::RootUnitOfWorkFactoryTrait;
use crate::root::dtos::{CreateRootDto, RootDto};
use anyhow::{Ok, Result};
use common::entities::Root;
use common::undo_redo::UndoRedoCommand;
use std::any::Any;
use std::collections::VecDeque;

pub struct CreateRootMultiUseCase {
    uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Root>>,
    redo_stack: VecDeque<Vec<Root>>,
}

impl CreateRootMultiUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>) -> Self {
        CreateRootMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }
    pub fn execute(&mut self, dtos: &[CreateRootDto]) -> Result<Vec<RootDto>> {
        // create
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let roots =
            uow.create_root_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        //store in undo stack
        self.undo_stack.push_back(roots.clone());
        self.redo_stack.clear();

        Ok(roots.into_iter().map(|root| root.into()).collect())
    }
}

impl UndoRedoCommand for CreateRootMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_roots) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_root_multi(
                &last_roots
                    .iter()
                    .map(|root| root.id.clone())
                    .collect::<Vec<_>>(),
            )?;
            uow.commit()?;
            self.redo_stack.push_back(last_roots);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_roots) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_root_multi(&last_roots)?;
            uow.commit()?;
            self.undo_stack.push_back(last_roots);
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
