use super::use_cases::{
    FeatureUnitOfWorkFactoryTrait, FeatureUnitOfWorkROFactoryTrait, FeatureUnitOfWorkROTrait,
    FeatureUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::Feature;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct FeatureUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl FeatureUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        FeatureUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for FeatureUnitOfWork {
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

#[macros::uow_action(entity = "Feature", action = "Create")]
#[macros::uow_action(entity = "Feature", action = "CreateMulti")]
#[macros::uow_action(entity = "Feature", action = "Get")]
#[macros::uow_action(entity = "Feature", action = "GetMulti")]
#[macros::uow_action(entity = "Feature", action = "Update")]
#[macros::uow_action(entity = "Feature", action = "UpdateMulti")]
#[macros::uow_action(entity = "Feature", action = "Delete")]
#[macros::uow_action(entity = "Feature", action = "DeleteMulti")]
#[macros::uow_action(entity = "Feature", action = "GetRelationship")]
#[macros::uow_action(entity = "Feature", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Feature", action = "SetRelationship")]
#[macros::uow_action(entity = "Feature", action = "SetRelationshipMulti")]
impl FeatureUnitOfWorkTrait for FeatureUnitOfWork {}

pub struct FeatureUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl FeatureUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        FeatureUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl FeatureUnitOfWorkFactoryTrait for FeatureUnitOfWorkFactory {
    fn create(&self) -> Box<dyn FeatureUnitOfWorkTrait> {
        Box::new(FeatureUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub struct FeatureUnitOfWorkRO {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl FeatureUnitOfWorkRO {
    pub fn new(db_context: &DbContext) -> Self {
        FeatureUnitOfWorkRO {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for FeatureUnitOfWorkRO {
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

#[macros::uow_action(entity = "Feature", action = "GetRO")]
#[macros::uow_action(entity = "Feature", action = "GetMultiRO")]
#[macros::uow_action(entity = "Feature", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Feature", action = "GetRelationshipsFromRightIdsRO")]
impl FeatureUnitOfWorkROTrait for FeatureUnitOfWorkRO {}

pub struct FeatureUnitOfWorkROFactory {
    context: DbContext,
}

impl FeatureUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        FeatureUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl FeatureUnitOfWorkROFactoryTrait for FeatureUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn FeatureUnitOfWorkROTrait> {
        Box::new(FeatureUnitOfWorkRO::new(&self.context))
    }
}
