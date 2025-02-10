use super::common::RootUnitOfWorkFactoryTrait;
use crate::root::dtos::RootDto;
use anyhow::{Ok, Result};
use common::{entities::Root, undo_redo::UndoRedoCommand};
use std::collections::VecDeque;

pub struct UpdateRootMultiUseCase {
    uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Root>>,
    redo_stack: VecDeque<Vec<Root>>,
}

impl UpdateRootMultiUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>) -> Self {
        UpdateRootMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dtos: &[RootDto]) -> Result<Vec<RootDto>> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let roots = uow.update_root_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(roots.clone());
        self.redo_stack.clear();

        Ok(roots.into_iter().map(|root| root.into()).collect())
    }
}

impl UndoRedoCommand for UpdateRootMultiUseCase {

    fn undo(&mut self) -> Result<()> {
        if let Some(last_roots) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            for root in &last_roots {
                uow.delete_root(&root.id)?;
            }
            uow.commit()?;
            self.redo_stack.push_back(last_roots);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_roots) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_root_multi(&last_roots)?;
            uow.commit()?;
            self.undo_stack.push_back(last_roots);
        }
        Ok(())
    }
}
