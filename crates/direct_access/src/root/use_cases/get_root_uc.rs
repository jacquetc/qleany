use anyhow::Result;
use common::{
    database::QueryUnitOfWork,
    entities::{EntityId, Root},
};
use super::common::RootUnitOfWorkROFactoryTrait;
use crate::root::dtos::RootDto;

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
