use anyhow::{Ok, Result, anyhow};
use crate::root::dtos::RootDto;
use super::common::RootUnitOfWorkTrait;

pub struct UpdateRootUseCase<'a> {
    uow: &'a mut dyn RootUnitOfWorkTrait,
}

impl<'a> UpdateRootUseCase<'a> {
    pub fn new(uow: &'a mut dyn RootUnitOfWorkTrait) -> Self {
        UpdateRootUseCase { uow }
    }

    pub fn execute(&mut self, dto: &RootDto) -> Result<RootDto> {
        self.uow.begin_transaction()?;

        // validate the dto
        if self.uow.get_root(&dto.id)?.is_none() {
            return Err(anyhow!("Root with id {} not found", dto.id));
        }

        let root = self.uow.update_root(&dto.into())?;
        self.uow.commit()?;
        Ok(root.into())
    }
}