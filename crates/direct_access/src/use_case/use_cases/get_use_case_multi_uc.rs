use super::UseCaseUnitOfWorkROFactoryTrait;
use crate::use_case::dtos::UseCaseDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetUseCaseMultiUseCase {
    uow_factory: Box<dyn UseCaseUnitOfWorkROFactoryTrait>,
}

impl GetUseCaseMultiUseCase {
    pub fn new(uow_factory: Box<dyn UseCaseUnitOfWorkROFactoryTrait>) -> Self {
        GetUseCaseMultiUseCase { uow_factory }
    }

    pub fn execute(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCaseDto>>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let use_cases = uow.get_use_case_multi(ids)?;
        uow.end_transaction()?;
        Ok(use_cases
            .into_iter()
            .map(|use_case| use_case.map(|uc| uc.into()))
            .collect())
    }
}
