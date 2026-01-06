use crate::use_cases::list_rust_files_uc::{
    ListRustFilesUnitOfWorkFactoryTrait, ListRustFilesUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::CommandUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::entities::Entity;
use common::entities::Feature;
use common::entities::File;
use common::entities::Global;
use common::entities::Relationship;
use common::entities::Root;
use common::entities::UseCase;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::sync::Arc;

// Unit of work for ListRustFiles

pub struct ListRustFilesUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl ListRustFilesUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        ListRustFilesUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for ListRustFilesUnitOfWork {
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
#[macros::uow_action(entity = "Root", action = "SetRelationship")]
#[macros::uow_action(entity = "Global", action = "Get")]
#[macros::uow_action(entity = "Entity", action = "GetMulti")]
#[macros::uow_action(entity = "Entity", action = "GetRelationship")]
#[macros::uow_action(entity = "Relationship", action = "GetMulti")]
#[macros::uow_action(entity = "Feature", action = "GetMulti")]
#[macros::uow_action(entity = "Feature", action = "GetRelationship")]
#[macros::uow_action(entity = "UseCase", action = "GetMulti")]
#[macros::uow_action(entity = "File", action = "Create")]
#[macros::uow_action(entity = "File", action = "CreateMulti")]
#[macros::uow_action(entity = "File", action = "DeleteMulti")]
impl ListRustFilesUnitOfWorkTrait for ListRustFilesUnitOfWork {}

pub struct ListRustFilesUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl ListRustFilesUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        ListRustFilesUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl ListRustFilesUnitOfWorkFactoryTrait for ListRustFilesUnitOfWorkFactory {
    fn create(&self) -> Box<dyn ListRustFilesUnitOfWorkTrait> {
        Box::new(ListRustFilesUnitOfWork::new(&self.context, &self.event_hub))
    }
}
