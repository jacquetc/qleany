use anyhow::{Ok, Result};
use crate::global::dtos::GlobalDto;
use super::common::GlobalUnitOfWorkTrait;

pub struct UpdateGlobalUseCase {
    uow: Box<dyn GlobalUnitOfWorkTrait>,
}

impl UpdateGlobalUseCase {
    pub fn new(uow: Box<dyn GlobalUnitOfWorkTrait>) -> Self {
        UpdateGlobalUseCase { uow }
    }

    pub fn execute(&mut self, dto: &GlobalDto) -> Result<GlobalDto> {
        self.uow.begin_transaction()?;
        let global = self.uow.update_global(&dto.into()).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(global.into())
    }
}
