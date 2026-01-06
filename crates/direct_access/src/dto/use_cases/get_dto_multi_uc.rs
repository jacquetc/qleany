use super::DtoUnitOfWorkROFactoryTrait;
use crate::dto::dtos::DtoDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetDtoMultiUseCase {
    uow_factory: Box<dyn DtoUnitOfWorkROFactoryTrait>,
}

impl GetDtoMultiUseCase {
    pub fn new(uow_factory: Box<dyn DtoUnitOfWorkROFactoryTrait>) -> Self {
        GetDtoMultiUseCase { uow_factory }
    }

    pub fn execute(&self, ids: &[EntityId]) -> Result<Vec<Option<DtoDto>>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let dtos = uow.get_dto_multi(ids)?;
        uow.end_transaction()?;
        Ok(dtos.into_iter().map(|dto| dto.map(|r| r.into())).collect())
    }
}
