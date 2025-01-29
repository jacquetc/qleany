use common::{database::QueryUnitOfWork, entities::{EntityId, Entity}};
use anyhow::Result;

use crate::entity::dtos::EntityDto;

pub trait EntityUnitOfWorkTraitRO : QueryUnitOfWork {
    fn get_entity(&self, id: &EntityId) -> Result<Option<Entity>>;
}

pub struct GetEntityUseCase {
    uow: Box<dyn EntityUnitOfWorkTraitRO>,
}

impl GetEntityUseCase {
    pub fn new(uow: Box<dyn EntityUnitOfWorkTraitRO>) -> Self {
        GetEntityUseCase { uow }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<EntityDto>> {
        self.uow.begin_transaction()?;
        let entity_option = self.uow.get_entity(&id).map_err(|e| {
            self.uow.end_transaction().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.end_transaction()?;

        Ok(entity_option.map(|entity| entity.into()))
    }
}
