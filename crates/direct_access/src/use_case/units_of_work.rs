use super::use_cases::{
    UseCaseUnitOfWorkFactoryTrait, UseCaseUnitOfWorkROFactoryTrait, UseCaseUnitOfWorkROTrait,
    UseCaseUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::entities::UseCase;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct UseCaseUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl UseCaseUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        UseCaseUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for UseCaseUnitOfWork {
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

#[macros::uow_action(entity = "UseCase", action = "Create")]
#[macros::uow_action(entity = "UseCase", action = "CreateMulti")]
#[macros::uow_action(entity = "UseCase", action = "Get")]
#[macros::uow_action(entity = "UseCase", action = "GetMulti")]
#[macros::uow_action(entity = "UseCase", action = "Update")]
#[macros::uow_action(entity = "UseCase", action = "UpdateMulti")]
#[macros::uow_action(entity = "UseCase", action = "Delete")]
#[macros::uow_action(entity = "UseCase", action = "DeleteMulti")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationship")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "UseCase", action = "SetRelationship")]
#[macros::uow_action(entity = "UseCase", action = "SetRelationshipMulti")]
impl UseCaseUnitOfWorkTrait for UseCaseUnitOfWork {}

pub struct UseCaseUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl UseCaseUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        UseCaseUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl UseCaseUnitOfWorkFactoryTrait for UseCaseUnitOfWorkFactory {
    fn create(&self) -> Box<dyn UseCaseUnitOfWorkTrait> {
        Box::new(UseCaseUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub struct UseCaseUnitOfWorkRO {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl UseCaseUnitOfWorkRO {
    pub fn new(db_context: &DbContext) -> Self {
        UseCaseUnitOfWorkRO {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for UseCaseUnitOfWorkRO {
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

#[macros::uow_action(entity = "UseCase", action = "GetRO")]
#[macros::uow_action(entity = "UseCase", action = "GetMultiRO")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationshipsFromRightIdsRO")]
impl UseCaseUnitOfWorkROTrait for UseCaseUnitOfWorkRO {}

pub struct UseCaseUnitOfWorkROFactory {
    context: DbContext,
}

impl UseCaseUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        UseCaseUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl UseCaseUnitOfWorkROFactoryTrait for UseCaseUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn UseCaseUnitOfWorkROTrait> {
        Box::new(UseCaseUnitOfWorkRO::new(&self.context))
    }
}
