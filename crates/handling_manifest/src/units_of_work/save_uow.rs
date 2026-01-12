use crate::use_cases::save_uc::{SaveUnitOfWorkFactoryTrait, SaveUnitOfWorkTrait};
use anyhow::{Ok, Result};
use common::database::QueryUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::entities::{Dto, DtoField, Entity, Feature, Field, Global, Root, UseCase, Workspace};
use common::event::EventHub;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct SaveUnitOfWork {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl SaveUnitOfWork {
    pub fn new(db_context: &DbContext) -> Self {
        SaveUnitOfWork {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for SaveUnitOfWork {
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

#[macros::uow_action(entity = "Root", action = "GetMultiRO")]
#[macros::uow_action(entity = "Root", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Workspace", action = "GetRO")]
#[macros::uow_action(entity = "Workspace", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Global", action = "GetRO")]
#[macros::uow_action(entity = "Feature", action = "GetMultiRO")]
#[macros::uow_action(entity = "Feature", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "UseCase", action = "GetMultiRO")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Entity", action = "GetMultiRO")]
#[macros::uow_action(entity = "Entity", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Field", action = "GetMultiRO")]
#[macros::uow_action(entity = "Field", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Dto", action = "GetMultiRO")]
#[macros::uow_action(entity = "Dto", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "DtoField", action = "GetMultiRO")]
impl SaveUnitOfWorkTrait for SaveUnitOfWork {}

pub struct SaveUnitOfWorkFactory {
    context: DbContext,
}

impl SaveUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        SaveUnitOfWorkFactory {
            context: db_context.clone(),
        }
    }
}

impl SaveUnitOfWorkFactoryTrait for SaveUnitOfWorkFactory {
    fn create(&self) -> Box<dyn SaveUnitOfWorkTrait> {
        Box::new(SaveUnitOfWork::new(&self.context))
    }
}
