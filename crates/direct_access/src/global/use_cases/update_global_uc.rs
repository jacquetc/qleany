use anyhow::{Ok, Result};
use crate::global::dtos::GlobalDto;
use super::common::GlobalUnitOfWorkTrait;

pub struct UpdateGlobalUseCase<'a> {
    uow: &'a mut dyn GlobalUnitOfWorkTrait,
}

impl<'a> UpdateGlobalUseCase<'a> {
    pub fn new(uow: &'a mut dyn GlobalUnitOfWorkTrait) -> Self {
        UpdateGlobalUseCase { uow }
    }

    pub fn execute(&mut self, dto: &GlobalDto) -> Result<GlobalDto> {
        self.uow.begin_transaction()?;
        let global = self.uow.update_global(&dto.into())?;
        self.uow.commit()?;
        Ok(global.into())
    }
}
