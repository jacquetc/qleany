use super::DtoFieldUnitOfWorkROFactoryTrait;
use crate::dto_field::dtos::DtoFieldDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetDtoFieldMultiUseCase {
    uow_factory: Box<dyn DtoFieldUnitOfWorkROFactoryTrait>,
}

impl GetDtoFieldMultiUseCase {
    pub fn new(uow_factory: Box<dyn DtoFieldUnitOfWorkROFactoryTrait>) -> Self {
        GetDtoFieldMultiUseCase { uow_factory }
    }

    pub fn execute(&self, ids: &[EntityId]) -> Result<Vec<Option<DtoFieldDto>>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let dto_fields = uow.get_dto_field_multi(ids)?;
        uow.end_transaction()?;
        Ok(dto_fields
            .into_iter()
            .map(|dto_field| dto_field.map(|r| r.into()))
            .collect())
    }
}
