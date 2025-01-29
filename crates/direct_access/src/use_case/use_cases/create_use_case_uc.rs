use anyhow::{Ok, Result};

use crate::use_case::dtos::{CreateUseCaseDto, UseCaseDto};

use super::common::UseCaseUnitOfWorkTrait;

pub struct CreateUseCaseUseCase<'a> {
    uow: &'a mut dyn UseCaseUnitOfWorkTrait,
}

impl<'a> CreateUseCaseUseCase<'a> {
    pub fn new(uow: &'a mut dyn UseCaseUnitOfWorkTrait) -> Self {
        CreateUseCaseUseCase { uow }
    }

    pub fn execute(&mut self, dto: CreateUseCaseDto) -> Result<UseCaseDto> {
        self.uow.begin_transaction()?;
        let use_case = self.uow.create_use_case(&dto.into())?;
        self.uow.commit()?;
        Ok(use_case.into())
    }
}
