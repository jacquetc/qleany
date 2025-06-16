use crate::use_cases::generate_rust_files_uc::{
    GenerateRustFilesUnitOfWorkFactoryTrait, GenerateRustFilesUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::QueryUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::direct_access::repository_factory;
use common::entities::Global;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct GenerateRustFilesUnitOfWork {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl GenerateRustFilesUnitOfWork {
    pub fn new(db_context: &DbContext) -> Self {
        GenerateRustFilesUnitOfWork {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for GenerateRustFilesUnitOfWork {
    fn begin_transaction(&self) -> Result<()> {
        self.transaction
            .replace(Some(Transaction::begin_read_transaction(&self.context)?));
        Ok(())
    }

    fn end_transaction(&self) -> Result<()> {
        self.transaction.take().unwrap().end_read_transaction()?;
        Ok(())
    }
}

#[macros::uow_action(entity = "Root", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Global", action = "GetMultiRO")]
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
