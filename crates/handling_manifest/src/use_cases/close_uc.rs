use anyhow::Result;
use common::types::EntityId;
use common::{database::CommandUnitOfWork, entities::Root};

use crate::use_cases::common::model_structs;

pub trait CloseUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn CloseUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "Delete")]
#[macros::uow_action(entity = "Root", action = "GetMulti")]
pub trait CloseUnitOfWorkTrait: CommandUnitOfWork {}

pub struct CloseUseCase {
    uow_factory: Box<dyn CloseUnitOfWorkFactoryTrait>,
}

impl CloseUseCase {
    pub fn new(uow_factory: Box<dyn CloseUnitOfWorkFactoryTrait>) -> Self {
        CloseUseCase { uow_factory }
    }

    pub fn execute(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Get all roots
        let roots = uow.get_root_multi(&[])?;
        if roots.is_empty() {
            return Err(anyhow::anyhow!("No root found"));
        }
        let root = &roots[0].as_ref().ok_or(anyhow::anyhow!("Root is None"))?;

        // Remove the root
        uow.delete_root(&root.id)?;

        uow.commit()?;

        Ok(())
    }
}
