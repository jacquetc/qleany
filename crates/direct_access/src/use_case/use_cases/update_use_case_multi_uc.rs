use super::UseCaseUnitOfWorkFactoryTrait;
use crate::use_case::dtos::UseCaseDto;
use anyhow::{Ok, Result};
use common::{entities::UseCase, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateUseCaseMultiUseCase {
    uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<UseCase>>,
    redo_stack: VecDeque<Vec<UseCase>>,
}

impl UpdateUseCaseMultiUseCase {
    pub fn new(uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>) -> Self {
        UpdateUseCaseMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dtos: &[UseCaseDto]) -> Result<Vec<UseCaseDto>> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // check if id exists
        let mut exists = true;
        for UseCaseDto { id, .. } in dtos {
            if uow.get_use_case(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }

        let use_cases =
            uow.update_use_case_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(use_cases.clone());
        self.redo_stack.clear();

        Ok(use_cases
            .into_iter()
            .map(|use_case| use_case.into())
            .collect())
    }
}

impl UndoRedoCommand for UpdateUseCaseMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_use_cases) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_use_case_multi(&last_use_cases)?;
            uow.commit()?;
            self.redo_stack.push_back(last_use_cases);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_use_cases) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_use_case_multi(&last_use_cases)?;
            uow.commit()?;
            self.undo_stack.push_back(last_use_cases);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
