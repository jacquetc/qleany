use common::{database::QueryUnitOfWork, entities::{EntityId, Global}};
use anyhow::Result;

use crate::global::dtos::GlobalDto;

pub trait GlobalUnitOfWorkTraitRO : QueryUnitOfWork {
    fn get_global(&self, id: &EntityId) -> Result<Option<Global>>;
}

pub struct GetGlobalUseCase {
    uow: Box<dyn GlobalUnitOfWorkTraitRO>,
}

impl GetGlobalUseCase {
    pub fn new(uow: Box<dyn GlobalUnitOfWorkTraitRO>) -> Self {
        GetGlobalUseCase { uow }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<GlobalDto>> {
        self.uow.begin_transaction()?;
        let global_option = self.uow.get_global(&id).map_err(|e| {
            self.uow.end_transaction().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.end_transaction()?;

        Ok(global_option.map(|global| global.into()))
    }
}
