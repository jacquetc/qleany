use super::RootUnitOfWorkFactoryTrait;
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

        // check if id exists
        let mut exists = true;
        for RootDto { id, .. } in dtos {
            if uow.get_root(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }

        let roots =
            uow.update_root_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
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
            uow.update_root_multi(&last_roots)?;
            uow.commit()?;
            self.redo_stack.push_back(last_roots);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(roots) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_root_multi(&roots)?;
            uow.commit()?;
            self.undo_stack.push_back(roots);
        }
        Ok(())
    }
}
