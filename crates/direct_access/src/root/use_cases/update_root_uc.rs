use super::common::RootUnitOfWorkFactoryTrait;
use crate::root::dtos::RootDto;
use anyhow::{anyhow, Ok, Result};

pub struct UpdateRootUseCase {
    uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>,
}

impl UpdateRootUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>) -> Self {
        UpdateRootUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &RootDto) -> Result<RootDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // validate the dto
        if uow.get_root(&dto.id)?.is_none() {
            return Err(anyhow!("Root with id {} not found", dto.id));
        }

        let root = uow.update_root(&dto.into())?;
        uow.commit()?;
        Ok(root.into())
    }
}
