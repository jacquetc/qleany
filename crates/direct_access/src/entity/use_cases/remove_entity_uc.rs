use super::common::{EntityUnitOfWorkFactoryTrait, EntityUnitOfWorkTrait};
use anyhow::{Ok, Result};
use common::entities::EntityId;

pub struct RemoveEntityUseCase {
    uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>,
}

impl RemoveEntityUseCase {
    pub fn new(uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>) -> Self {
        RemoveEntityUseCase { uow_factory }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.delete_entity(&id)?;
        uow.commit()?;
        Ok(())
    }
}
