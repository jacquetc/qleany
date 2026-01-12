use crate::use_cases::load_uc::{LoadUnitOfWorkFactoryTrait, LoadUnitOfWorkTrait};
use anyhow::{Ok, Result};
use common::database::CommandUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::entities::{
    Dto, DtoField, Entity, Feature, Field, Global, Relationship, Root, UseCase, Workspace, System, UserInterface,
};
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::sync::Arc;

pub struct LoadUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl LoadUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        LoadUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for LoadUnitOfWork {
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

#[macros::uow_action(entity = "Root", action = "Create")]
#[macros::uow_action(entity = "Root", action = "Get")]
#[macros::uow_action(entity = "Root", action = "Update")]
#[macros::uow_action(entity = "Workspace", action = "Create")]
#[macros::uow_action(entity = "Workspace", action = "Get")]
#[macros::uow_action(entity = "Workspace", action = "Update")]
#[macros::uow_action(entity = "System", action = "Create")]
#[macros::uow_action(entity = "System", action = "Get")]
#[macros::uow_action(entity = "System", action = "Update")]
#[macros::uow_action(entity = "Global", action = "Create")]
#[macros::uow_action(entity = "Feature", action = "Create")]
#[macros::uow_action(entity = "UseCase", action = "Create")]
#[macros::uow_action(entity = "Entity", action = "Create")]
#[macros::uow_action(entity = "Entity", action = "Get")]
#[macros::uow_action(entity = "Entity", action = "Update")]
#[macros::uow_action(entity = "Field", action = "Create")]
#[macros::uow_action(entity = "Field", action = "GetMulti")]
#[macros::uow_action(entity = "Dto", action = "Create")]
#[macros::uow_action(entity = "DtoField", action = "Create")]
#[macros::uow_action(entity = "Relationship", action = "CreateMulti")]
#[macros::uow_action(entity = "UserInterface", action = "Create")]
impl LoadUnitOfWorkTrait for LoadUnitOfWork {}

pub struct LoadUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl LoadUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        LoadUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl LoadUnitOfWorkFactoryTrait for LoadUnitOfWorkFactory {
    fn create(&self) -> Box<dyn LoadUnitOfWorkTrait> {
        Box::new(LoadUnitOfWork::new(&self.context, &self.event_hub))
    }
}
