use anyhow::{Ok, Result};
use common::entities::EntityId;
use super::common::RootUnitOfWorkTrait;

pub struct RemoveRootUseCase<'a> {
    uow: &'a mut dyn RootUnitOfWorkTrait,
}

impl<'a> RemoveRootUseCase<'a> {
    pub fn new(uow: &'a mut dyn RootUnitOfWorkTrait) -> Self {
        RemoveRootUseCase { uow }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        self.uow.begin_transaction()?;
        self.uow.delete_root(&id)?;
        self.uow.commit()?;
        Ok(())
    }
}