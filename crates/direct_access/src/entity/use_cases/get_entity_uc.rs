use super::EntityUnitOfWorkROFactoryTrait;
use crate::entity::dtos::EntityDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetEntityUseCase {
    uow_factory: Box<dyn EntityUnitOfWorkROFactoryTrait>,
}

impl GetEntityUseCase {
    pub fn new(uow_factory: Box<dyn EntityUnitOfWorkROFactoryTrait>) -> Self {
        GetEntityUseCase { uow_factory }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<EntityDto>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let entity_option = uow.get_entity(&id)?;
        uow.end_transaction()?;

        Ok(entity_option.map(|entity| entity.into()))
    }
}
