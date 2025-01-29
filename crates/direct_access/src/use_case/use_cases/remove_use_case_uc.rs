use anyhow::{Ok, Result};
use common::entities::EntityId;
use super::common::UseCaseUnitOfWorkTrait;

pub struct RemoveUseCaseUseCase<'a> {
    uow: &'a mut dyn UseCaseUnitOfWorkTrait,
}

impl<'a> RemoveUseCaseUseCase<'a> {
    pub fn new(uow: &'a mut dyn UseCaseUnitOfWorkTrait) -> Self {
        RemoveUseCaseUseCase { uow }
    }

    pub fn execute(&mut self, id: &EntityId) -> Result<()> {
        self.uow.begin_transaction()?;
        self.uow.delete_use_case(&id)?;
        self.uow.commit()?;
        Ok(())
    }
}
