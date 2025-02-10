use super::common::RootUnitOfWorkROFactoryTrait;
use crate::root::dtos::RootDto;
use anyhow::Result;
use common::entities::EntityId;

pub struct GetRootMultiUseCase {
    uow_factory: Box<dyn RootUnitOfWorkROFactoryTrait>,
}

impl GetRootMultiUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkROFactoryTrait>) -> Self {
        GetRootMultiUseCase { uow_factory }
    }

    pub fn execute(&self, ids: &[EntityId]) -> Result<Vec<Option<RootDto>>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let roots = uow.get_root_multi(ids)?;
        uow.end_transaction()?;
        Ok(roots
            .into_iter()
            .map(|root| root.map(|r| r.into()))
            .collect())
    }
}
