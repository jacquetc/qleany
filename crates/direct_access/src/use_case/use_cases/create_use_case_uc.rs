use anyhow::{Ok, Result};

use crate::use_case::dtos::{CreateUseCaseDto, UseCaseDto};

use super::common::UseCaseUnitOfWorkFactoryTrait;

pub struct CreateUseCaseUseCase {
    uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>,
}

impl CreateUseCaseUseCase {
    pub fn new(uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>) -> Self {
        CreateUseCaseUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: CreateUseCaseDto) -> Result<UseCaseDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let use_case = uow.create_use_case(&dto.into())?;
        uow.commit()?;
        Ok(use_case.into())
    }
}
