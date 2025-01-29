use super::common::{EntityUnitOfWorkFactoryTrait, EntityUnitOfWorkTrait};
use crate::entity::dtos::EntityDto;
use anyhow::{Ok, Result};

pub struct UpdateEntityUseCase {
    uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>,
}

impl UpdateEntityUseCase {
    pub fn new(uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>) -> Self {
        UpdateEntityUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &EntityDto) -> Result<EntityDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let entity = uow.update_entity(&dto.into())?;
        uow.commit()?;
        Ok(entity.into())
    }
}
