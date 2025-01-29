use common::{database::QueryUnitOfWork, entities::{EntityId, Entity}};
use anyhow::Result;

use crate::entity::dtos::EntityDto;

pub trait EntityUnitOfWorkTraitRO : QueryUnitOfWork {
    fn get_entity(&self, id: &EntityId) -> Result<Option<Entity>>;
}

pub struct GetEntityUseCase<'a> {
    uow: &'a dyn EntityUnitOfWorkTraitRO,
}

impl<'a> GetEntityUseCase<'a> {
    pub fn new(uow: &'a dyn EntityUnitOfWorkTraitRO) -> Self {
        GetEntityUseCase { uow }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<EntityDto>> {
        self.uow.begin_transaction()?;
        let entity_option = self.uow.get_entity(&id)?;
        self.uow.end_transaction()?;

        Ok(entity_option.map(|entity| entity.into()))
    }
}
