use super::common::UseCaseUnitOfWorkROFactoryTrait;
use crate::use_case::dtos::UseCaseDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetUseCaseUseCase {
    uow_factory: Box<dyn UseCaseUnitOfWorkROFactoryTrait>,
}

impl GetUseCaseUseCase {
    pub fn new(uow_factory: Box<dyn UseCaseUnitOfWorkROFactoryTrait>) -> Self {
        GetUseCaseUseCase { uow_factory }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<UseCaseDto>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let use_case_option = uow.get_use_case(&id)?;
        uow.end_transaction()?;

        Ok(use_case_option.map(|use_case| use_case.into()))
    }
}
