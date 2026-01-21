use crate::use_cases::save_uc::{SaveUnitOfWorkFactoryTrait, SaveUnitOfWorkTrait};
use anyhow::{Ok, Result};
use common::database::CommandUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::entities::UserInterface;
use common::entities::{Dto, DtoField, Entity, Feature, Field, Global, Root, UseCase, Workspace};
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types::{self, EntityId};
use std::sync::Arc;

pub struct SaveUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl SaveUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        SaveUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for SaveUnitOfWork {
    fn begin_transaction(&mut self) -> Result<()> {
        self.transaction = Some(Transaction::begin_write_transaction(&self.context)?);
        Ok(())
    }

    fn commit(&mut self) -> Result<()> {
        self.transaction.take().unwrap().commit()?;
        Ok(())
    }

    fn rollback(&mut self) -> Result<()> {
        self.transaction.take().unwrap().rollback()?;
        Ok(())
    }

    fn create_savepoint(&self) -> Result<types::Savepoint> {
        self.transaction.as_ref().unwrap().create_savepoint()
    }

    fn restore_to_savepoint(&mut self, savepoint: types::Savepoint) -> Result<()> {
        let mut transaction = self.transaction.take().unwrap();
        transaction.restore_to_savepoint(savepoint)?;

        self.event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::All(AllEvent::Reset)),
            ids: vec![],
            data: None,
        });

        // Recreate the transaction after restoring to savepoint
        self.transaction = Some(transaction);

        Ok(())
    }
}

#[macros::uow_action(entity = "Root", action = "GetMulti")]
#[macros::uow_action(entity = "Root", action = "GetRelationship")]
#[macros::uow_action(entity = "Workspace", action = "Get")]
#[macros::uow_action(entity = "Workspace", action = "Update")]
#[macros::uow_action(entity = "Workspace", action = "GetRelationship")]
#[macros::uow_action(entity = "Global", action = "Get")]
#[macros::uow_action(entity = "UserInterface", action = "Get")]
#[macros::uow_action(entity = "Feature", action = "GetMulti")]
#[macros::uow_action(entity = "Feature", action = "GetRelationship")]
#[macros::uow_action(entity = "UseCase", action = "GetMulti")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationship")]
#[macros::uow_action(entity = "Entity", action = "GetMulti")]
#[macros::uow_action(entity = "Entity", action = "GetRelationship")]
#[macros::uow_action(entity = "Field", action = "GetMulti")]
#[macros::uow_action(entity = "Field", action = "GetRelationship")]
#[macros::uow_action(entity = "Dto", action = "GetMulti")]
#[macros::uow_action(entity = "Dto", action = "GetRelationship")]
#[macros::uow_action(entity = "DtoField", action = "GetMulti")]
impl SaveUnitOfWorkTrait for SaveUnitOfWork {}

pub struct SaveUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl SaveUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        SaveUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl SaveUnitOfWorkFactoryTrait for SaveUnitOfWorkFactory {
    fn create(&self) -> Box<dyn SaveUnitOfWorkTrait> {
        Box::new(SaveUnitOfWork::new(&self.context, &self.event_hub))
    }
}
