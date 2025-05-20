use super::common::GlobalUnitOfWorkFactoryTrait;
use anyhow::{Ok, Result};
use common::types::EntityId;

pub struct RemoveGlobalUseCase {
    uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>,
}

impl RemoveGlobalUseCase {
    pub fn new(uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>) -> Self {
        RemoveGlobalUseCase { uow_factory }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.delete_global(&id)?;
        uow.commit()?;
        Ok(())
    }
}
