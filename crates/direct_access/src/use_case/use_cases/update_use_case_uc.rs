use super::UseCaseUnitOfWorkFactoryTrait;
use crate::use_case::dtos::UseCaseDto;
use anyhow::{Ok, Result};
use common::{entities::UseCase, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateUseCaseUseCase {
    uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<UseCase>,
    redo_stack: VecDeque<UseCase>,
}

impl UpdateUseCaseUseCase {
    pub fn new(uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>) -> Self {
        UpdateUseCaseUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn description(&self) -> &str {
        "Update Use Case"
    }

    pub fn execute(&mut self, dto: &UseCaseDto) -> Result<UseCaseDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // check if id exists
        if uow.get_use_case(&dto.id)?.is_none() {
            return Err(anyhow::anyhow!("Root with id {} does not exist", dto.id));
        }

        let use_case = uow.update_use_case(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(use_case.clone());
        self.redo_stack.clear();

        Ok(use_case.into())
    }
}

impl UndoRedoCommand for UpdateUseCaseUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_use_case) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_use_case(&last_use_case)?;
            uow.commit()?;
            self.redo_stack.push_back(last_use_case);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_use_case) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_use_case(&last_use_case)?;
            uow.commit()?;
            self.undo_stack.push_back(last_use_case);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
