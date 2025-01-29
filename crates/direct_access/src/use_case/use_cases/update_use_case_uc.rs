use anyhow::{Ok, Result};
use crate::use_case::dtos::UseCaseDto;
use super::common::UseCaseUnitOfWorkTrait;

pub struct UpdateUseCaseUseCase {
    uow: Box<dyn UseCaseUnitOfWorkTrait>,
}

impl UpdateUseCaseUseCase {
    pub fn new(uow: Box<dyn UseCaseUnitOfWorkTrait>) -> Self {
        UpdateUseCaseUseCase { uow }
    }

    pub fn execute(&mut self, dto: &UseCaseDto) -> Result<UseCaseDto> {
        self.uow.begin_transaction()?;
        let use_case = self.uow.update_use_case(&dto.into()).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(use_case.into())
    }
}
