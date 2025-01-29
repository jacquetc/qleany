use anyhow::Result;
use common::{
    database::QueryUnitOfWork,
    entities::{Entity, EntityId},
};

use crate::entity::dtos::EntityDto;

pub trait EntityUnitOfWorkROTrait: QueryUnitOfWork {
    fn get_entity(&self, id: &EntityId) -> Result<Option<Entity>>;
}

pub trait EntityUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn EntityUnitOfWorkROTrait>;
}

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
