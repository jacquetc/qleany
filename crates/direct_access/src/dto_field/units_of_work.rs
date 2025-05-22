use super::use_cases::{
    DtoFieldUnitOfWorkFactoryTrait, DtoFieldUnitOfWorkROFactoryTrait, DtoFieldUnitOfWorkROTrait,
    DtoFieldUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::DtoField;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct DtoFieldUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl DtoFieldUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        DtoFieldUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for DtoFieldUnitOfWork {
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

        Ok(())
    }
}

#[macros::uow_action(entity = "DtoField", action = "Create")]
#[macros::uow_action(entity = "DtoField", action = "CreateMulti")]
#[macros::uow_action(entity = "DtoField", action = "Get")]
#[macros::uow_action(entity = "DtoField", action = "GetMulti")]
#[macros::uow_action(entity = "DtoField", action = "Update")]
#[macros::uow_action(entity = "DtoField", action = "UpdateMulti")]
#[macros::uow_action(entity = "DtoField", action = "Delete")]
#[macros::uow_action(entity = "DtoField", action = "DeleteMulti")]
impl DtoFieldUnitOfWorkTrait for DtoFieldUnitOfWork {}

pub struct DtoFieldUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl DtoFieldUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        DtoFieldUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl DtoFieldUnitOfWorkFactoryTrait for DtoFieldUnitOfWorkFactory {
    fn create(&self) -> Box<dyn DtoFieldUnitOfWorkTrait> {
        Box::new(DtoFieldUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub struct DtoFieldUnitOfWorkRO {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl DtoFieldUnitOfWorkRO {
    pub fn new(db_context: &DbContext) -> Self {
        DtoFieldUnitOfWorkRO {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for DtoFieldUnitOfWorkRO {
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

#[macros::uow_action(entity = "DtoField", action = "GetRO")]
#[macros::uow_action(entity = "DtoField", action = "GetMultiRO")]
impl DtoFieldUnitOfWorkROTrait for DtoFieldUnitOfWorkRO {}

pub struct DtoFieldUnitOfWorkROFactory {
    context: DbContext,
}

impl DtoFieldUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        DtoFieldUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl DtoFieldUnitOfWorkROFactoryTrait for DtoFieldUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn DtoFieldUnitOfWorkROTrait> {
        Box::new(DtoFieldUnitOfWorkRO::new(&self.context))
    }
}
