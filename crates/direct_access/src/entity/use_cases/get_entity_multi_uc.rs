use super::EntityUnitOfWorkROFactoryTrait;
use crate::entity::dtos::EntityDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetEntityMultiUseCase {
    uow_factory: Box<dyn EntityUnitOfWorkROFactoryTrait>,
}

impl GetEntityMultiUseCase {
    pub fn new(uow_factory: Box<dyn EntityUnitOfWorkROFactoryTrait>) -> Self {
        GetEntityMultiUseCase { uow_factory }
    }

    pub fn execute(&self, ids: &[EntityId]) -> Result<Vec<Option<EntityDto>>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let entitys = uow.get_entity_multi(ids)?;
        uow.end_transaction()?;
        Ok(entitys
            .into_iter()
            .map(|entity| entity.map(|r| r.into()))
            .collect())
    }
}
