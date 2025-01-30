use anyhow::{Ok, Result};

use crate::global::dtos::{CreateGlobalDto, GlobalDto};

use super::common::GlobalUnitOfWorkFactoryTrait;

pub struct CreateGlobalUseCase {
    uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>,
}

impl CreateGlobalUseCase {
    pub fn new(uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>) -> Self {
        CreateGlobalUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: CreateGlobalDto) -> Result<GlobalDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let global = uow.create_global(&dto.into())?;
        uow.commit()?;
        Ok(global.into())
    }
}
