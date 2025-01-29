use anyhow::{Ok, Result};

use crate::use_case::dtos::{CreateUseCaseDto, UseCaseDto};

use super::common::UseCaseUnitOfWorkTrait;

pub struct CreateUseCaseUseCase {
    uow: Box<dyn UseCaseUnitOfWorkTrait>,
}

impl CreateUseCaseUseCase {
    pub fn new(uow: Box<dyn UseCaseUnitOfWorkTrait>) -> Self {
        CreateUseCaseUseCase { uow }
    }

    pub fn execute(&mut self, dto: CreateUseCaseDto) -> Result<UseCaseDto> {
        self.uow.begin_transaction()?;
        let use_case = self.uow.create_use_case(&dto.into()).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(use_case.into())
    }
}
