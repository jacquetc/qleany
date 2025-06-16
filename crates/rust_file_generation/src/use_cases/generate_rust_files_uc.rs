use anyhow::Result;
use common::types::EntityId;
use common::{
    database::QueryUnitOfWork,
    entities::{
        Dto, DtoField, Entity, Feature, Field, FieldType, Global, Relationship, Root, UseCase,
    },
};

use crate::GenerateRustFilesDto;

pub trait GenerateRustFilesUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn GenerateRustFilesUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Global", action = "GetMultiRO")]
pub trait GenerateRustFilesUnitOfWorkTrait: QueryUnitOfWork {}

pub struct GenerateRustFilesUseCase {
    uow_factory: Box<dyn GenerateRustFilesUnitOfWorkFactoryTrait>,
}

impl GenerateRustFilesUseCase {
    pub fn new(uow_factory: Box<dyn GenerateRustFilesUnitOfWorkFactoryTrait>) -> Self {
        GenerateRustFilesUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &GenerateRustFilesDto) -> Result<()> {
        Ok(())
    }
}
