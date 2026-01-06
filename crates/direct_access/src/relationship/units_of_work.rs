use super::use_cases::{
    RelationshipUnitOfWorkFactoryTrait, RelationshipUnitOfWorkROFactoryTrait,
    RelationshipUnitOfWorkROTrait, RelationshipUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::entities::Relationship;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct RelationshipUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl RelationshipUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        RelationshipUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for RelationshipUnitOfWork {
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

#[macros::uow_action(entity = "Relationship", action = "Create")]
#[macros::uow_action(entity = "Relationship", action = "CreateMulti")]
#[macros::uow_action(entity = "Relationship", action = "Get")]
#[macros::uow_action(entity = "Relationship", action = "GetMulti")]
#[macros::uow_action(entity = "Relationship", action = "Update")]
#[macros::uow_action(entity = "Relationship", action = "UpdateMulti")]
#[macros::uow_action(entity = "Relationship", action = "Delete")]
#[macros::uow_action(entity = "Relationship", action = "DeleteMulti")]
#[macros::uow_action(entity = "Relationship", action = "GetRelationship")]
#[macros::uow_action(entity = "Relationship", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Relationship", action = "SetRelationship")]
#[macros::uow_action(entity = "Relationship", action = "SetRelationshipMulti")]
impl RelationshipUnitOfWorkTrait for RelationshipUnitOfWork {}

pub struct RelationshipUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl RelationshipUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        RelationshipUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl RelationshipUnitOfWorkFactoryTrait for RelationshipUnitOfWorkFactory {
    fn create(&self) -> Box<dyn RelationshipUnitOfWorkTrait> {
        Box::new(RelationshipUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub struct RelationshipUnitOfWorkRO {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl RelationshipUnitOfWorkRO {
    pub fn new(db_context: &DbContext) -> Self {
        RelationshipUnitOfWorkRO {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for RelationshipUnitOfWorkRO {
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

#[macros::uow_action(entity = "Relationship", action = "GetRO")]
#[macros::uow_action(entity = "Relationship", action = "GetMultiRO")]
#[macros::uow_action(entity = "Relationship", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Relationship", action = "GetRelationshipsFromRightIdsRO")]
impl RelationshipUnitOfWorkROTrait for RelationshipUnitOfWorkRO {}

pub struct RelationshipUnitOfWorkROFactory {
    context: DbContext,
}

impl RelationshipUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        RelationshipUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl RelationshipUnitOfWorkROFactoryTrait for RelationshipUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn RelationshipUnitOfWorkROTrait> {
        Box::new(RelationshipUnitOfWorkRO::new(&self.context))
    }
}
