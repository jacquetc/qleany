use common::{database::QueryUnitOfWork, entities::{EntityId, UseCase}};
use anyhow::Result;

use crate::use_case::dtos::UseCaseDto;

pub trait UseCaseUnitOfWorkTraitRO : QueryUnitOfWork {
    fn get_use_case(&self, id: &EntityId) -> Result<Option<UseCase>>;
}

pub struct GetUseCaseUseCase<'a> {
    uow: &'a dyn UseCaseUnitOfWorkTraitRO,
}

impl<'a> GetUseCaseUseCase<'a> {
    pub fn new(uow: &'a dyn UseCaseUnitOfWorkTraitRO) -> Self {
        GetUseCaseUseCase { uow }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<UseCaseDto>> {
        self.uow.begin_transaction()?;
        let use_case_option = self.uow.get_use_case(&id)?;
        self.uow.end_transaction()?;

        Ok(use_case_option.map(|use_case| use_case.into()))
    }
}
