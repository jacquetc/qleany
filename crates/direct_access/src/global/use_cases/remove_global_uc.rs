use anyhow::{Ok, Result};
use common::entities::EntityId;
use super::common::GlobalUnitOfWorkTrait;

pub struct RemoveGlobalUseCase<'a> {
    uow: &'a mut dyn GlobalUnitOfWorkTrait,
}

impl<'a> RemoveGlobalUseCase<'a> {
    pub fn new(uow: &'a mut dyn GlobalUnitOfWorkTrait) -> Self {
        RemoveGlobalUseCase { uow }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        self.uow.begin_transaction()?;
        self.uow.delete_global(&id)?;
        self.uow.commit()?;
        Ok(())
    }
}
