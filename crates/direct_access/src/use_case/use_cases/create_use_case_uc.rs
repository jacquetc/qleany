use anyhow::{Ok, Result};
use crate::use_case::dtos::{CreateUseCaseDto, UseCaseDto};
use super::common::UseCaseUnitOfWorkFactoryTrait;
use common::entities::UseCase;
use std::collections::VecDeque;

pub struct CreateUseCaseUseCase {
    uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<UseCase>,
    redo_stack: VecDeque<UseCase>,
}

impl CreateUseCaseUseCase {
    pub fn new(uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>) -> Self {
        CreateUseCaseUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: CreateUseCaseDto) -> Result<UseCaseDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let use_case = uow.create_use_case(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(use_case.clone());
        self.redo_stack.clear();

        Ok(use_case.into())
    }

    pub fn undo(&mut self) -> Result<()> {
        if let Some(last_use_case) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_use_case(&last_use_case.id)?;
            uow.commit()?;
            self.redo_stack.push_back(last_use_case);
        }
        Ok(())
    }

    pub fn redo(&mut self) -> Result<()> {
        if let Some(last_use_case) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_use_case(&last_use_case)?;
            uow.commit()?;
            self.undo_stack.push_back(last_use_case);
        }
        Ok(())
    }
}
