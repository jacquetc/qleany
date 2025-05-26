use anyhow::Result;
use common::types::EntityId;
use common::{
    database::CommandUnitOfWork,
    entities::{
        Dto, DtoField, Entity, Feature, Field, FieldType, Global, Relationship, Root, UseCase,
    },
};

use crate::GenerateRustBaseFilesDto;

pub trait GenerateRustBaseFilesUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn GenerateRustBaseFilesUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetRelationship")]
#[macros::uow_action(entity = "Root", action = "SetRelationship")]
#[macros::uow_action(entity = "Global", action = "GetMulti")]
pub trait GenerateRustBaseFilesUnitOfWorkTrait: CommandUnitOfWork {}

pub struct GenerateRustBaseFilesUseCase {
    uow_factory: Box<dyn GenerateRustBaseFilesUnitOfWorkFactoryTrait>,
}

impl GenerateRustBaseFilesUseCase {
    pub fn new(uow_factory: Box<dyn GenerateRustBaseFilesUnitOfWorkFactoryTrait>) -> Self {
        GenerateRustBaseFilesUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &GenerateRustBaseFilesDto) -> Result<()> {
        Ok(())
    }
}
