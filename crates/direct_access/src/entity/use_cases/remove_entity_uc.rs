use anyhow::{Ok, Result};
use common::entities::EntityId;
use super::common::EntityUnitOfWorkTrait;

pub struct RemoveEntityUseCase<'a> {
    uow: &'a mut dyn EntityUnitOfWorkTrait,
}

impl<'a> RemoveEntityUseCase<'a> {
    pub fn new(uow: &'a mut dyn EntityUnitOfWorkTrait) -> Self {
        RemoveEntityUseCase { uow }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        self.uow.begin_transaction()?;
        self.uow.delete_entity(&id)?;
        self.uow.commit()?;
        Ok(())
    }
}
