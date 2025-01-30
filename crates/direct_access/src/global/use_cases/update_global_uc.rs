use super::common::GlobalUnitOfWorkFactoryTrait;
use crate::global::dtos::GlobalDto;
use anyhow::{Ok, Result};

pub struct UpdateGlobalUseCase {
    uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>,
}

impl UpdateGlobalUseCase {
    pub fn new(uow_factory: Box<dyn GlobalUnitOfWorkFactoryTrait>) -> Self {
        UpdateGlobalUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &GlobalDto) -> Result<GlobalDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let global = uow.update_global(&dto.into())?;
        uow.commit()?;
        Ok(global.into())
    }
}
