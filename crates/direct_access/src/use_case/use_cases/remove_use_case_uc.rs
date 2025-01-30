use super::common::UseCaseUnitOfWorkFactoryTrait;
use anyhow::{Ok, Result};
use common::entities::EntityId;

pub struct RemoveUseCaseUseCase {
    uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>,
}

impl RemoveUseCaseUseCase {
    pub fn new(uow_factory: Box<dyn UseCaseUnitOfWorkFactoryTrait>) -> Self {
        RemoveUseCaseUseCase { uow_factory }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.delete_use_case(&id)?;
        uow.commit()?;
        Ok(())
    }
}
