use crate::use_cases::common::rust_code_generator::GenerationReadOps;
use crate::use_cases::generate_rust_files_uc::{
    GenerateRustFilesUnitOfWorkFactoryTrait, GenerateRustFilesUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::QueryUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::entities::{Dto, DtoField, Entity, Feature, Field, File, Global, Root, UseCase};
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::sync::Mutex;

pub struct GenerateRustFilesUnitOfWork {
    context: DbContext,
    transaction: Mutex<Option<Transaction>>,
}

impl GenerateRustFilesUnitOfWork {
    pub fn new(db_context: &DbContext) -> Self {
        GenerateRustFilesUnitOfWork {
            context: db_context.clone(),
            transaction: Mutex::new(None),
        }
    }
}

impl QueryUnitOfWork for GenerateRustFilesUnitOfWork {
    fn begin_transaction(&self) -> Result<()> {
        let mut transaction = self.transaction.lock().unwrap();
        *transaction = Some(Transaction::begin_read_transaction(&self.context)?);
        Ok(())
    }

    fn end_transaction(&self) -> Result<()> {
        let mut transaction = self.transaction.lock().unwrap();
        transaction.take().unwrap().end_read_transaction()?;
        Ok(())
    }
}

#[macros::uow_action(entity = "Root", action = "GetRelationshipRO", thread_safe = true)]
#[macros::uow_action(entity = "File", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "Feature", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "UseCase", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "UseCase", action = "GetMultiRO", thread_safe = true)]
#[macros::uow_action(entity = "Dto", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "DtoField", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "DtoField", action = "GetMultiRO", thread_safe = true)]
#[macros::uow_action(entity = "Entity", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "Entity", action = "GetMultiRO", thread_safe = true)]
#[macros::uow_action(entity = "Field", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "Field", action = "GetMultiRO", thread_safe = true)]
impl GenerationReadOps for GenerateRustFilesUnitOfWork {}

#[macros::uow_action(entity = "Root", action = "GetRelationshipRO", thread_safe = true)]
#[macros::uow_action(entity = "Global", action = "GetMultiRO", thread_safe = true)]
impl GenerateRustFilesUnitOfWorkTrait for GenerateRustFilesUnitOfWork {}

pub struct GenerateRustFilesUnitOfWorkFactory {
    context: DbContext,
}

impl GenerateRustFilesUnitOfWorkFactory {
    pub fn new(db_context: &DbContext) -> Self {
        GenerateRustFilesUnitOfWorkFactory {
            context: db_context.clone(),
        }
    }
}

impl GenerateRustFilesUnitOfWorkFactoryTrait for GenerateRustFilesUnitOfWorkFactory {
    fn create(&self) -> Box<dyn GenerateRustFilesUnitOfWorkTrait> {
        Box::new(GenerateRustFilesUnitOfWork::new(&self.context))
    }
}
