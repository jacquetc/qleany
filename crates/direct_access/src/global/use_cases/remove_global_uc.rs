use anyhow::{Ok, Result};
use common::entities::EntityId;
use super::common::GlobalUnitOfWorkTrait;

pub struct RemoveGlobalUseCase {
    uow: Box<dyn GlobalUnitOfWorkTrait>,
}

impl RemoveGlobalUseCase {
    pub fn new(uow: Box<dyn GlobalUnitOfWorkTrait>) -> Self {
        RemoveGlobalUseCase { uow }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        self.uow.begin_transaction()?;
        self.uow.delete_global(&id).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(())
    }
}
