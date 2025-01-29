use anyhow::{Ok, Result};
use common::entities::EntityId;
use super::common::UseCaseUnitOfWorkTrait;

pub struct RemoveUseCaseUseCase {
    uow: Box<dyn UseCaseUnitOfWorkTrait>,
}

impl RemoveUseCaseUseCase {
    pub fn new(uow: Box<dyn UseCaseUnitOfWorkTrait>) -> Self {
        RemoveUseCaseUseCase { uow }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        self.uow.begin_transaction()?;
        self.uow.delete_use_case(&id).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(())
    }
}
