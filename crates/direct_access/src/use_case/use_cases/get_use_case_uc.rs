use anyhow::Result;
use common::{
    database::QueryUnitOfWork,
    entities::{EntityId, UseCase},
};

use crate::use_case::dtos::UseCaseDto;

pub trait UseCaseUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn UseCaseUnitOfWorkROTrait>;
}

pub trait UseCaseUnitOfWorkROTrait: QueryUnitOfWork {
    fn get_use_case(&self, id: &EntityId) -> Result<Option<UseCase>>;
}

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
