use anyhow::{Ok, Result};

use crate::root::dtos::{CreateRootDto, RootDto};

use super::common::RootUnitOfWorkTrait;

pub struct CreateRootUseCase<'a> {
    uow: &'a mut dyn RootUnitOfWorkTrait,
}

impl<'a> CreateRootUseCase<'a> {
    pub fn new(uow: &'a mut dyn RootUnitOfWorkTrait) -> Self {
        CreateRootUseCase { uow }
    }

    pub fn execute(&mut self, dto: CreateRootDto) -> Result<RootDto> {
        self.uow.begin_transaction()?;
        let root = self.uow.create_root(&dto.into())?;
        self.uow.commit()?;
        Ok(root.into())
    }
}