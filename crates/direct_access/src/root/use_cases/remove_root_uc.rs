use anyhow::{Ok, Result};
use common::entities::EntityId;
use super::common::RootUnitOfWorkTrait;

pub struct RemoveRootUseCase {
    uow: Box<dyn RootUnitOfWorkTrait>,
}

impl RemoveRootUseCase {
    pub fn new(uow: Box<dyn RootUnitOfWorkTrait>) -> Self {
        RemoveRootUseCase { uow }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        self.uow.begin_transaction()?;
        self.uow.delete_root(&id).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(())
    }
}