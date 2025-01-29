use common::{database::QueryUnitOfWork, entities::{EntityId, Root}};
use anyhow::Result;

use crate::root::dtos::RootDto;

pub trait RootUnitOfWorkTraitRO : QueryUnitOfWork {
    fn get_root(&self, id: &EntityId) -> Result<Option<Root>>;
}

pub struct GetRootUseCase<'a> {
    uow: &'a dyn RootUnitOfWorkTraitRO,
}

impl<'a> GetRootUseCase<'a> {
    pub fn new(uow: &'a dyn RootUnitOfWorkTraitRO) -> Self {
        GetRootUseCase { uow }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<RootDto>> {
        self.uow.begin_transaction()?;
        let root_option = self.uow.get_root(&id)?;
        self.uow.end_transaction()?;

        Ok(root_option.map(|root| root.into()))
    }
}