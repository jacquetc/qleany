use anyhow::Result;

use crate::entity::dtos::{CreateEntityDto, EntityDto};

use super::common::EntityUnitOfWorkTrait;

pub struct CreateEntityUseCase {
    uow: Box<dyn EntityUnitOfWorkTrait>,
}

impl CreateEntityUseCase {
    pub fn new(uow: Box<dyn EntityUnitOfWorkTrait>) -> Self {
        CreateEntityUseCase { uow }
    }

    pub fn execute(&mut self, dto: CreateEntityDto) -> Result<EntityDto> {
        self.uow.begin_transaction()?;
        let entity = self.uow.create_entity(&dto.into()).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(entity.into())
    }
}
