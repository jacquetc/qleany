use super::common::RootUnitOfWorkFactoryTrait;
use anyhow::{Ok, Result};
use common::entities::EntityId;

pub struct RemoveRootUseCase {
    uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>,
}

impl RemoveRootUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>) -> Self {
        RemoveRootUseCase { uow_factory }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.delete_root(&id)?;
        uow.commit()?;
        Ok(())
    }
}
