use super::use_cases::{
    FieldUnitOfWorkFactoryTrait, FieldUnitOfWorkROFactoryTrait, FieldUnitOfWorkROTrait,
    FieldUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::entities::Field;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct FieldUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl FieldUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        FieldUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for FieldUnitOfWork {
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

#[macros::uow_action(entity = "Field", action = "Create")]
#[macros::uow_action(entity = "Field", action = "CreateMulti")]
#[macros::uow_action(entity = "Field", action = "Get")]
#[macros::uow_action(entity = "Field", action = "GetMulti")]
#[macros::uow_action(entity = "Field", action = "Update")]
#[macros::uow_action(entity = "Field", action = "UpdateMulti")]
#[macros::uow_action(entity = "Field", action = "Delete")]
#[macros::uow_action(entity = "Field", action = "DeleteMulti")]
#[macros::uow_action(entity = "Field", action = "GetRelationship")]
#[macros::uow_action(entity = "Field", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Field", action = "SetRelationship")]
#[macros::uow_action(entity = "Field", action = "SetRelationshipMulti")]
impl FieldUnitOfWorkTrait for FieldUnitOfWork {}

pub struct FieldUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl FieldUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        FieldUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl FieldUnitOfWorkFactoryTrait for FieldUnitOfWorkFactory {
    fn create(&self) -> Box<dyn FieldUnitOfWorkTrait> {
        Box::new(FieldUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub struct FieldUnitOfWorkRO {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl FieldUnitOfWorkRO {
    pub fn new(db_context: &DbContext) -> Self {
        FieldUnitOfWorkRO {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for FieldUnitOfWorkRO {
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

#[macros::uow_action(entity = "Field", action = "GetRO")]
#[macros::uow_action(entity = "Field", action = "GetMultiRO")]
#[macros::uow_action(entity = "Field", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Field", action = "GetRelationshipsFromRightIdsRO")]
impl FieldUnitOfWorkROTrait for FieldUnitOfWorkRO {}

pub struct FieldUnitOfWorkROFactory {
    context: DbContext,
}

impl FieldUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        FieldUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl FieldUnitOfWorkROFactoryTrait for FieldUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn FieldUnitOfWorkROTrait> {
        Box::new(FieldUnitOfWorkRO::new(&self.context))
    }
}
