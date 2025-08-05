use crate::GenerateRustFilesDto;
use anyhow::Result;
use common::long_operation::LongOperation;
use common::types::EntityId;
use common::{
    database::QueryUnitOfWork,
    entities::{
        Dto, DtoField, Entity, Feature, Field, FieldType, Global, Relationship, Root, UseCase,
    },
};
use std::sync::Arc;

pub trait GenerateRustFilesUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn GenerateRustFilesUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Global", action = "GetMultiRO")]
pub trait GenerateRustFilesUnitOfWorkTrait: QueryUnitOfWork + Send + Sync {}

pub struct GenerateRustFilesUseCase {
    uow_factory: Box<dyn GenerateRustFilesUnitOfWorkFactoryTrait>,
    dto: GenerateRustFilesDto,
}

impl GenerateRustFilesUseCase {
    pub fn new(
        uow_factory: Box<dyn GenerateRustFilesUnitOfWorkFactoryTrait>,
        dto: &GenerateRustFilesDto,
    ) -> Self {
        GenerateRustFilesUseCase {
            uow_factory,
            dto: dto.clone(),
        }
    }
}
impl LongOperation for GenerateRustFilesUseCase {
    fn execute(
        &self,
        progress_callback: Box<dyn Fn(common::long_operation::OperationProgress) + Send>,
        _cancel_flag: Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<()> {
        // Report initial progress
        progress_callback(common::long_operation::OperationProgress::new(
            0.0,
            Some("Starting Rust file generation...".to_string()),
        ));

        Ok(())
    }
}
