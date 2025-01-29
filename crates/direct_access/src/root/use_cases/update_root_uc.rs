use anyhow::{Ok, Result, anyhow};
use crate::root::dtos::RootDto;
use super::common::RootUnitOfWorkTrait;

pub struct UpdateRootUseCase {
    uow: Box<dyn RootUnitOfWorkTrait>,
}

impl UpdateRootUseCase {
    pub fn new(uow: Box<dyn RootUnitOfWorkTrait>) -> Self {
        UpdateRootUseCase { uow }
    }

    pub fn execute(&mut self, dto: &RootDto) -> Result<RootDto> {
        self.uow.begin_transaction()?;

        // validate the dto
        if self.uow.get_root(&dto.id)?.is_none() {
            return Err(anyhow!("Root with id {} not found", dto.id));
        }

        let root = self.uow.update_root(&dto.into()).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(root.into())
    }
}