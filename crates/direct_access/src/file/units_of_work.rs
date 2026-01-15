use super::use_cases::{
    FileUnitOfWorkFactoryTrait, FileUnitOfWorkROFactoryTrait, FileUnitOfWorkROTrait,
    FileUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::entities::File;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct FileUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl FileUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        FileUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for FileUnitOfWork {
    fn begin_transaction(&mut self) -> Result<()> {
        self.transaction = Some(Transaction::begin_write_transaction(&self.context)?);
        Ok(())
    }

    fn commit(&mut self) -> Result<()> {
        let transaction = self.transaction.take();
        let mut transaction = transaction.unwrap();
        transaction.commit()?;
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

#[macros::uow_action(entity = "File", action = "Create")]
#[macros::uow_action(entity = "File", action = "CreateMulti")]
#[macros::uow_action(entity = "File", action = "Get")]
#[macros::uow_action(entity = "File", action = "GetMulti")]
#[macros::uow_action(entity = "File", action = "Update")]
#[macros::uow_action(entity = "File", action = "UpdateMulti")]
#[macros::uow_action(entity = "File", action = "Delete")]
#[macros::uow_action(entity = "File", action = "DeleteMulti")]
impl FileUnitOfWorkTrait for FileUnitOfWork {}

pub struct FileUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl FileUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        FileUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl FileUnitOfWorkFactoryTrait for FileUnitOfWorkFactory {
    fn create(&self) -> Box<dyn FileUnitOfWorkTrait> {
        Box::new(FileUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub struct FileUnitOfWorkRO {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl FileUnitOfWorkRO {
    pub fn new(db_context: &DbContext) -> Self {
        FileUnitOfWorkRO {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for FileUnitOfWorkRO {
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

#[macros::uow_action(entity = "File", action = "GetRO")]
#[macros::uow_action(entity = "File", action = "GetMultiRO")]
impl FileUnitOfWorkROTrait for FileUnitOfWorkRO {}

pub struct FileUnitOfWorkROFactory {
    context: DbContext,
}

impl FileUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        FileUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl FileUnitOfWorkROFactoryTrait for FileUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn FileUnitOfWorkROTrait> {
        Box::new(FileUnitOfWorkRO::new(&self.context))
    }
}
