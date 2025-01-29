use anyhow::{Ok, Result};

use crate::entity::dtos::{CreateEntityDto, EntityDto};

use super::common::EntityUnitOfWorkTrait;

pub struct CreateEntityUseCase<'a> {
    uow: &'a mut dyn EntityUnitOfWorkTrait,
}

impl<'a> CreateEntityUseCase<'a> {
    pub fn new(uow: &'a mut dyn EntityUnitOfWorkTrait) -> Self {
        CreateEntityUseCase { uow }
    }

    pub fn execute(&mut self, dto: CreateEntityDto) -> Result<EntityDto> {
        self.uow.begin_transaction()?;
        let entity = self.uow.create_entity(&dto.into())?;
        self.uow.commit()?;
        Ok(entity.into())
    }
}
