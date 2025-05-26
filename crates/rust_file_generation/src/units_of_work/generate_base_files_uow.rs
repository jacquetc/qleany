use crate::use_cases::generate_rust_base_files_uc::{
    GenerateRustBaseFilesUnitOfWorkFactoryTrait, GenerateRustBaseFilesUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::CommandUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::direct_access::repository_factory;
use common::entities::Global;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::sync::Arc;

pub struct GenerateRustBaseFilesUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl GenerateRustBaseFilesUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        GenerateRustBaseFilesUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for GenerateRustBaseFilesUnitOfWork {
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

#[macros::uow_action(entity = "Root", action = "GetRelationship")]
#[macros::uow_action(entity = "Root", action = "SetRelationship")]
#[macros::uow_action(entity = "Global", action = "GetMulti")]
impl GenerateRustBaseFilesUnitOfWorkTrait for GenerateRustBaseFilesUnitOfWork {}

pub struct GenerateRustBaseFilesUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl GenerateRustBaseFilesUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        GenerateRustBaseFilesUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl GenerateRustBaseFilesUnitOfWorkFactoryTrait for GenerateRustBaseFilesUnitOfWorkFactory {
    fn create(&self) -> Box<dyn GenerateRustBaseFilesUnitOfWorkTrait> {
        Box::new(GenerateRustBaseFilesUnitOfWork::new(
            &self.context,
            &self.event_hub,
        ))
    }
}
