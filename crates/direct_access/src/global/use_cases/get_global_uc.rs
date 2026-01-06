use super::GlobalUnitOfWorkROFactoryTrait;
use crate::global::dtos::GlobalDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetGlobalUseCase {
    uow_factory: Box<dyn GlobalUnitOfWorkROFactoryTrait>,
}

impl GetGlobalUseCase {
    pub fn new(uow_factory: Box<dyn GlobalUnitOfWorkROFactoryTrait>) -> Self {
        GetGlobalUseCase { uow_factory }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<GlobalDto>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let global_option = uow.get_global(&id)?;
        uow.end_transaction()?;

        Ok(global_option.map(|global| global.into()))
    }
}
