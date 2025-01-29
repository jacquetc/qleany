use anyhow::{Ok, Result};

use crate::global::dtos::{CreateGlobalDto, GlobalDto};

use super::common::GlobalUnitOfWorkTrait;

pub struct CreateGlobalUseCase {
    uow: Box<dyn GlobalUnitOfWorkTrait>,
}

impl CreateGlobalUseCase {
    pub fn new(uow: Box<dyn GlobalUnitOfWorkTrait>) -> Self {
        CreateGlobalUseCase { uow }
    }

    pub fn execute(&mut self, dto: CreateGlobalDto) -> Result<GlobalDto> {
        self.uow.begin_transaction()?;
        let global = self.uow.create_global(&dto.into()).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(global.into())
    }
}
