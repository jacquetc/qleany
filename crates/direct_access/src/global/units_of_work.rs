use super::use_cases::{
    GlobalUnitOfWorkFactoryTrait, GlobalUnitOfWorkROFactoryTrait, GlobalUnitOfWorkROTrait,
    GlobalUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::Global;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct GlobalUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl GlobalUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        GlobalUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for GlobalUnitOfWork {
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

#[macros::uow_action(entity = "Global", action = "Create")]
#[macros::uow_action(entity = "Global", action = "CreateMulti")]
#[macros::uow_action(entity = "Global", action = "Get")]
#[macros::uow_action(entity = "Global", action = "GetMulti")]
#[macros::uow_action(entity = "Global", action = "Update")]
#[macros::uow_action(entity = "Global", action = "UpdateMulti")]
#[macros::uow_action(entity = "Global", action = "Delete")]
#[macros::uow_action(entity = "Global", action = "DeleteMulti")]
impl GlobalUnitOfWorkTrait for GlobalUnitOfWork {}

pub struct GlobalUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl GlobalUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        GlobalUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl GlobalUnitOfWorkFactoryTrait for GlobalUnitOfWorkFactory {
    fn create(&self) -> Box<dyn GlobalUnitOfWorkTrait> {
        Box::new(GlobalUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub struct GlobalUnitOfWorkRO {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl GlobalUnitOfWorkRO {
    pub fn new(db_context: &DbContext) -> Self {
        GlobalUnitOfWorkRO {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for GlobalUnitOfWorkRO {
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

#[macros::uow_action(entity = "Global", action = "GetRO")]
#[macros::uow_action(entity = "Global", action = "GetMultiRO")]
impl GlobalUnitOfWorkROTrait for GlobalUnitOfWorkRO {}

pub struct GlobalUnitOfWorkROFactory {
    context: DbContext,
}

impl GlobalUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        GlobalUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl GlobalUnitOfWorkROFactoryTrait for GlobalUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn GlobalUnitOfWorkROTrait> {
        Box::new(GlobalUnitOfWorkRO::new(&self.context))
    }
}
