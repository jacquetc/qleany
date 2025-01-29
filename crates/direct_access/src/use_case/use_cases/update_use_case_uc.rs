use anyhow::{Ok, Result};
use crate::use_case::dtos::UseCaseDto;
use super::common::UseCaseUnitOfWorkTrait;

pub struct UpdateUseCaseUseCase<'a> {
    uow: &'a mut dyn UseCaseUnitOfWorkTrait,
}

impl<'a> UpdateUseCaseUseCase<'a> {
    pub fn new(uow: &'a mut dyn UseCaseUnitOfWorkTrait) -> Self {
        UpdateUseCaseUseCase { uow }
    }

    pub fn execute(&mut self, dto: &UseCaseDto) -> Result<UseCaseDto> {
        self.uow.begin_transaction()?;
        let use_case = self.uow.update_use_case(&dto.into())?;
        self.uow.commit()?;
        Ok(use_case.into())
    }
}
