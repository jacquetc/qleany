use super::use_cases::{
    EntityUnitOfWorkFactoryTrait, EntityUnitOfWorkROFactoryTrait, EntityUnitOfWorkROTrait,
    EntityUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::Entity;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct EntityUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl EntityUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        EntityUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for EntityUnitOfWork {
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

#[macros::uow_action(entity = "Entity", action = "Create")]
#[macros::uow_action(entity = "Entity", action = "CreateMulti")]
#[macros::uow_action(entity = "Entity", action = "Get")]
#[macros::uow_action(entity = "Entity", action = "GetMulti")]
#[macros::uow_action(entity = "Entity", action = "Update")]
#[macros::uow_action(entity = "Entity", action = "UpdateMulti")]
#[macros::uow_action(entity = "Entity", action = "Delete")]
#[macros::uow_action(entity = "Entity", action = "DeleteMulti")]
#[macros::uow_action(entity = "Entity", action = "GetRelationship")]
#[macros::uow_action(entity = "Entity", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Entity", action = "SetRelationship")]
#[macros::uow_action(entity = "Entity", action = "SetRelationshipMulti")]
impl EntityUnitOfWorkTrait for EntityUnitOfWork {}

pub struct EntityUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl EntityUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        EntityUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl EntityUnitOfWorkFactoryTrait for EntityUnitOfWorkFactory {
    fn create(&self) -> Box<dyn EntityUnitOfWorkTrait> {
        Box::new(EntityUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub struct EntityUnitOfWorkRO {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl EntityUnitOfWorkRO {
    pub fn new(db_context: &DbContext) -> Self {
        EntityUnitOfWorkRO {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for EntityUnitOfWorkRO {
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

#[macros::uow_action(entity = "Entity", action = "GetRO")]
#[macros::uow_action(entity = "Entity", action = "GetMultiRO")]
#[macros::uow_action(entity = "Entity", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Entity", action = "GetRelationshipsFromRightIdsRO")]
impl EntityUnitOfWorkROTrait for EntityUnitOfWorkRO {}

pub struct EntityUnitOfWorkROFactory {
    context: DbContext,
}

impl EntityUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        EntityUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl EntityUnitOfWorkROFactoryTrait for EntityUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn EntityUnitOfWorkROTrait> {
        Box::new(EntityUnitOfWorkRO::new(&self.context))
    }
}
