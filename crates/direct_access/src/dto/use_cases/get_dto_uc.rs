use super::DtoUnitOfWorkROFactoryTrait;
use crate::dto::dtos::DtoDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetDtoUseCase {
    uow_factory: Box<dyn DtoUnitOfWorkROFactoryTrait>,
}

impl GetDtoUseCase {
    pub fn new(uow_factory: Box<dyn DtoUnitOfWorkROFactoryTrait>) -> Self {
        GetDtoUseCase { uow_factory }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<DtoDto>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let dto_option = uow.get_dto(&id)?;
        uow.end_transaction()?;

        Ok(dto_option.map(|dto| dto.into()))
    }
}
