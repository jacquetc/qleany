use super::FieldUnitOfWorkROFactoryTrait;
use crate::field::dtos::FieldDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetFieldUseCase {
    uow_factory: Box<dyn FieldUnitOfWorkROFactoryTrait>,
}

impl GetFieldUseCase {
    pub fn new(uow_factory: Box<dyn FieldUnitOfWorkROFactoryTrait>) -> Self {
        GetFieldUseCase { uow_factory }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<FieldDto>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let field_option = uow.get_field(&id)?;
        uow.end_transaction()?;

        Ok(field_option.map(|field| field.into()))
    }
}
