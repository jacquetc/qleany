use super::DtoFieldUnitOfWorkROFactoryTrait;
use crate::dto_field::dtos::DtoFieldDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetDtoFieldUseCase {
    uow_factory: Box<dyn DtoFieldUnitOfWorkROFactoryTrait>,
}

impl GetDtoFieldUseCase {
    pub fn new(uow_factory: Box<dyn DtoFieldUnitOfWorkROFactoryTrait>) -> Self {
        GetDtoFieldUseCase { uow_factory }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<DtoFieldDto>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let dto_field_option = uow.get_dto_field(&id)?;
        uow.end_transaction()?;

        Ok(dto_field_option.map(|dto_field| dto_field.into()))
    }
}
