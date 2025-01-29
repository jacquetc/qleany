use anyhow::{Ok, Result};

use crate::global::dtos::{CreateGlobalDto, GlobalDto};

use super::common::GlobalUnitOfWorkTrait;

pub struct CreateGlobalUseCase<'a> {
    uow: &'a mut dyn GlobalUnitOfWorkTrait,
}

impl<'a> CreateGlobalUseCase<'a> {
    pub fn new(uow: &'a mut dyn GlobalUnitOfWorkTrait) -> Self {
        CreateGlobalUseCase { uow }
    }

    pub fn execute(&mut self, dto: CreateGlobalDto) -> Result<GlobalDto> {
        self.uow.begin_transaction()?;
        let global = self.uow.create_global(&dto.into())?;
        self.uow.commit()?;
        Ok(global.into())
    }
}
