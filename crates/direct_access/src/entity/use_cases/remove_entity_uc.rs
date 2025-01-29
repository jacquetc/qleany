use anyhow::{Ok, Result};
use common::entities::EntityId;
use super::common::EntityUnitOfWorkTrait;

pub struct RemoveEntityUseCase {
    uow: Box<dyn EntityUnitOfWorkTrait>,
}

impl RemoveEntityUseCase {
    pub fn new(uow: Box<dyn EntityUnitOfWorkTrait>) -> Self {
        RemoveEntityUseCase { uow }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        self.uow.begin_transaction()?;
        self.uow.delete_entity(&id).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(())
    }
}
