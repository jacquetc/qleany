use anyhow::Result;
use common::entities::Entity;
use common::types::EntityId;
use common::{database::CommandUnitOfWork, entities::Global};

use crate::ListRustFilesDto;

pub trait ListRustFilesUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn ListRustFilesUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetRelationship")]
#[macros::uow_action(entity = "Root", action = "SetRelationship")]
#[macros::uow_action(entity = "Global", action = "GetMulti")]
#[macros::uow_action(entity = "Entity", action = "GetMulti")]
pub trait ListRustFilesUnitOfWorkTrait: CommandUnitOfWork {}

pub struct ListRustFilesUseCase {
    uow_factory: Box<dyn ListRustFilesUnitOfWorkFactoryTrait>,
}

impl ListRustFilesUseCase {
    pub fn new(uow_factory: Box<dyn ListRustFilesUnitOfWorkFactoryTrait>) -> Self {
        ListRustFilesUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &ListRustFilesDto) -> Result<()> {
        Ok(())
    }
}
