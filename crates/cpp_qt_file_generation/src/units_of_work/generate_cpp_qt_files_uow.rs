use crate::use_cases::common::cpp_qt_code_generator::GenerationReadOps;
use crate::use_cases::generate_cpp_qt_files_uc::{
    GenerateCppQtFilesUnitOfWorkFactoryTrait, GenerateCppQtFilesUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::QueryUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::entities::UserInterface;
use common::entities::Workspace;
use common::entities::{
    Dto, DtoField, Entity, Feature, Field, File, Global, Relationship, Root, UseCase,
};
use common::types::EntityId;
use std::sync::Mutex;

pub struct GenerateCppQtFilesUnitOfWork {
    context: DbContext,
    transaction: Mutex<Option<Transaction>>,
}

impl GenerateCppQtFilesUnitOfWork {
    pub fn new(db_context: &DbContext) -> Self {
        GenerateCppQtFilesUnitOfWork {
            context: db_context.clone(),
            transaction: Mutex::new(None),
        }
    }
}

impl QueryUnitOfWork for GenerateCppQtFilesUnitOfWork {
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
#[macros::uow_action(entity = "Root", action = "GetMultiRO", thread_safe = true)]
#[macros::uow_action(entity = "Workspace", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "Workspace", action = "GetRelationshipRO", thread_safe = true)]
#[macros::uow_action(entity = "File", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "Global", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "UserInterface", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "Feature", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "Feature", action = "GetMultiRO", thread_safe = true)]
#[macros::uow_action(entity = "UseCase", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "UseCase", action = "GetMultiRO", thread_safe = true)]
#[macros::uow_action(entity = "Dto", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "DtoField", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "DtoField", action = "GetMultiRO", thread_safe = true)]
#[macros::uow_action(entity = "Entity", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "Entity", action = "GetMultiRO", thread_safe = true)]
#[macros::uow_action(entity = "Field", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "Field", action = "GetMultiRO", thread_safe = true)]
#[macros::uow_action(entity = "Relationship", action = "GetRO", thread_safe = true)]
#[macros::uow_action(entity = "Relationship", action = "GetMultiRO", thread_safe = true)]
impl GenerationReadOps for GenerateCppQtFilesUnitOfWork {}

#[macros::uow_action(entity = "Root", action = "GetRelationshipRO", thread_safe = true)]
#[macros::uow_action(entity = "Root", action = "GetMultiRO", thread_safe = true)]
#[macros::uow_action(entity = "Global", action = "GetMultiRO", thread_safe = true)]
impl GenerateCppQtFilesUnitOfWorkTrait for GenerateCppQtFilesUnitOfWork {}

pub struct GenerateCppQtFilesUnitOfWorkFactory {
    context: DbContext,
}

impl GenerateCppQtFilesUnitOfWorkFactory {
    pub fn new(db_context: &DbContext) -> Self {
        GenerateCppQtFilesUnitOfWorkFactory {
            context: db_context.clone(),
        }
    }
}

impl GenerateCppQtFilesUnitOfWorkFactoryTrait for GenerateCppQtFilesUnitOfWorkFactory {
    fn create(&self) -> Box<dyn GenerateCppQtFilesUnitOfWorkTrait> {
        Box::new(GenerateCppQtFilesUnitOfWork::new(&self.context))
    }
}
