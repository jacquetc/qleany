use anyhow::{Ok, Result};

use crate::root::dtos::{CreateRootDto, RootDto};

use super::common::RootUnitOfWorkTrait;

pub struct CreateRootUseCase {
    uow: Box<dyn RootUnitOfWorkTrait>,
}

impl CreateRootUseCase {
    pub fn new(uow: Box<dyn RootUnitOfWorkTrait>) -> Self {
        CreateRootUseCase { uow }
    }

    pub fn execute(&mut self, dto: CreateRootDto) -> Result<RootDto> {
        self.uow.begin_transaction()?;
        let root = self.uow.create_root(&dto.into()).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(root.into())
    }
}