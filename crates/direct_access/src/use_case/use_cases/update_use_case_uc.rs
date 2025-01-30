use super::common::UseCaseUnitOfWorkFactoryTrait;
use crate::use_case::dtos::UseCaseDto;
use anyhow::{Ok, Result};

pub struct UpdateUseCaseUseCase {
    uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>,
}

impl UpdateUseCaseUseCase {
    pub fn new(uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>) -> Self {
        UpdateUseCaseUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &UseCaseDto) -> Result<UseCaseDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let use_case = uow.update_use_case(&dto.into())?;
        uow.commit()?;
        Ok(use_case.into())
    }
}
