use super::RootUnitOfWorkROFactoryTrait;
use crate::root::dtos::RootDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetRootUseCase {
    uow_factory: Box<dyn RootUnitOfWorkROFactoryTrait>,
}

impl GetRootUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkROFactoryTrait>) -> Self {
        GetRootUseCase { uow_factory }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<RootDto>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let root_option = uow.get_root(&id)?;
        uow.end_transaction()?;

        Ok(root_option.map(|root| root.into()))
    }
}
