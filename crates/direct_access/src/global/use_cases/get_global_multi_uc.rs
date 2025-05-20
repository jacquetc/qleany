use super::GlobalUnitOfWorkROFactoryTrait;
use crate::global::dtos::GlobalDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetGlobalMultiUseCase {
    uow_factory: Box<dyn GlobalUnitOfWorkROFactoryTrait>,
}

impl GetGlobalMultiUseCase {
    pub fn new(uow_factory: Box<dyn GlobalUnitOfWorkROFactoryTrait>) -> Self {
        GetGlobalMultiUseCase { uow_factory }
    }

    pub fn execute(&self, ids: &[EntityId]) -> Result<Vec<Option<GlobalDto>>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let globals = uow.get_global_multi(ids)?;
        uow.end_transaction()?;
        Ok(globals
            .into_iter()
            .map(|global| global.map(|r| r.into()))
            .collect())
    }
}
