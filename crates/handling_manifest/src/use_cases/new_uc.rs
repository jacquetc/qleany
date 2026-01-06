use anyhow::{anyhow, Result};
use common::database::CommandUnitOfWork;
use common::entities::{Entity, Field, Root};
use common::types::EntityId;

pub trait NewUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn NewUnitOfWorkTrait>;
}
#[macros::uow_action(entity = "Root", action = "Get")]
#[macros::uow_action(entity = "Root", action = "GetMulti")]
#[macros::uow_action(entity = "Entity", action = "Get")]
#[macros::uow_action(entity = "Entity", action = "GetMulti")]
#[macros::uow_action(entity = "Field", action = "Get")]
#[macros::uow_action(entity = "Field", action = "GetMulti")]
pub trait NewUnitOfWorkTrait: CommandUnitOfWork {}

pub struct NewUseCase {
    uow_factory: Box<dyn NewUnitOfWorkFactoryTrait>,
}

impl NewUseCase {
    pub fn new(uow_factory: Box<dyn NewUnitOfWorkFactoryTrait>) -> Self {
        NewUseCase { uow_factory }
    }

    pub fn execute(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        //TODO: NewUseCase to be implemented
        unimplemented!("NewUseCase unimplemented");
        uow.commit()?;
        Ok(())
    }
}
