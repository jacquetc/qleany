use anyhow::Result;

use crate::entity::dtos::{CreateEntityDto, EntityDto};

use super::common::EntityUnitOfWorkFactoryTrait;

pub struct CreateEntityUseCase {
    uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>,
}

impl CreateEntityUseCase {
    pub fn new(uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>) -> Self {
        CreateEntityUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: CreateEntityDto) -> Result<EntityDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let entity = uow.create_entity(&dto.into())?;
        uow.commit()?;
        Ok(entity.into())
    }
}
