use anyhow::{Ok, Result};
use crate::entity::dtos::EntityDto;
use super::common::EntityUnitOfWorkTrait;

pub struct UpdateEntityUseCase<'a> {
    uow: &'a mut dyn EntityUnitOfWorkTrait,
}

impl<'a> UpdateEntityUseCase<'a> {
    pub fn new(uow: &'a mut dyn EntityUnitOfWorkTrait) -> Self {
        UpdateEntityUseCase { uow }
    }

    pub fn execute(&mut self, dto: &EntityDto) -> Result<EntityDto> {
        self.uow.begin_transaction()?;
        let entity = self.uow.update_entity(&dto.into())?;
        self.uow.commit()?;
        Ok(entity.into())
    }
}
