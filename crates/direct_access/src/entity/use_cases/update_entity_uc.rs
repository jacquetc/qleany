use anyhow::{Ok, Result};
use crate::entity::dtos::EntityDto;
use super::common::EntityUnitOfWorkTrait;

pub struct UpdateEntityUseCase {
    uow: Box<dyn EntityUnitOfWorkTrait>,
}

impl UpdateEntityUseCase {
    pub fn new(uow: Box<dyn EntityUnitOfWorkTrait>) -> Self {
        UpdateEntityUseCase { uow }
    }

    pub fn execute(&mut self, dto: &EntityDto) -> Result<EntityDto> {
        self.uow.begin_transaction()?;
        let entity = self.uow.update_entity(&dto.into()).map_err(|e| {
            self.uow.rollback().unwrap_or_else(|_| ());
            e
        })?;
        self.uow.commit()?;
        Ok(entity.into())
    }
}
