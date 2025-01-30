use anyhow::{Ok, Result};

use crate::root::dtos::{CreateRootDto, RootDto};

use super::common::RootUnitOfWorkFactoryTrait;

pub struct CreateRootUseCase {
    uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>,
}

impl CreateRootUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>) -> Self {
        CreateRootUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: CreateRootDto) -> Result<RootDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let root = uow.create_root(&dto.into())?;
        uow.commit()?;
        Ok(root.into())
    }
}
