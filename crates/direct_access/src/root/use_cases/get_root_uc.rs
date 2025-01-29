use common::{database::QueryUnitOfWork, entities::{EntityId, Root}};
use anyhow::Result;

use crate::root::dtos::RootDto;

pub trait RootUnitOfWorkTraitRO : QueryUnitOfWork {
    fn get_root(&self, id: &EntityId) -> Result<Option<Root>>;
}

pub struct GetRootUseCase {
    uow: Box<dyn RootUnitOfWorkTraitRO>,
}

impl GetRootUseCase {
    pub fn new(uow: Box<dyn RootUnitOfWorkTraitRO>) -> Self {
        GetRootUseCase { uow }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<RootDto>> {
        self.uow.begin_transaction()?;
        let root_option = self.uow.get_root(&id).map_err(|e| {
            self.uow.end_transaction().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.end_transaction()?;

        Ok(root_option.map(|root| root.into()))
    }
}