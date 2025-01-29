use common::{database::QueryUnitOfWork, entities::{EntityId, UseCase}};
use anyhow::Result;

use crate::use_case::dtos::UseCaseDto;

pub trait UseCaseUnitOfWorkTraitRO : QueryUnitOfWork {
    fn get_use_case(&self, id: &EntityId) -> Result<Option<UseCase>>;
}

pub struct GetUseCaseUseCase {
    uow: Box<dyn UseCaseUnitOfWorkTraitRO>,
}

impl GetUseCaseUseCase {
    pub fn new(uow: Box<dyn UseCaseUnitOfWorkTraitRO>) -> Self {
        GetUseCaseUseCase { uow }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<UseCaseDto>> {
        self.uow.begin_transaction()?;
        let use_case_option = self.uow.get_use_case(&id).map_err(|e| {
            self.uow.end_transaction().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.end_transaction()?;

        Ok(use_case_option.map(|use_case| use_case.into()))
    }
}
