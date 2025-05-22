use super::use_cases::{
    DtoUnitOfWorkFactoryTrait, DtoUnitOfWorkROFactoryTrait, DtoUnitOfWorkROTrait,
    DtoUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::Dto;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct DtoUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl DtoUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        DtoUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for DtoUnitOfWork {
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

#[macros::uow_action(entity = "Dto", action = "Create")]
#[macros::uow_action(entity = "Dto", action = "CreateMulti")]
#[macros::uow_action(entity = "Dto", action = "Get")]
#[macros::uow_action(entity = "Dto", action = "GetMulti")]
#[macros::uow_action(entity = "Dto", action = "Update")]
#[macros::uow_action(entity = "Dto", action = "UpdateMulti")]
#[macros::uow_action(entity = "Dto", action = "Delete")]
#[macros::uow_action(entity = "Dto", action = "DeleteMulti")]
#[macros::uow_action(entity = "Dto", action = "GetRelationship")]
#[macros::uow_action(entity = "Dto", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Dto", action = "SetRelationship")]
#[macros::uow_action(entity = "Dto", action = "SetRelationshipMulti")]
impl DtoUnitOfWorkTrait for DtoUnitOfWork {}

pub struct DtoUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl DtoUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        DtoUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl DtoUnitOfWorkFactoryTrait for DtoUnitOfWorkFactory {
    fn create(&self) -> Box<dyn DtoUnitOfWorkTrait> {
        Box::new(DtoUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub struct DtoUnitOfWorkRO {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl DtoUnitOfWorkRO {
    pub fn new(db_context: &DbContext) -> Self {
        DtoUnitOfWorkRO {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for DtoUnitOfWorkRO {
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

#[macros::uow_action(entity = "Dto", action = "GetRO")]
#[macros::uow_action(entity = "Dto", action = "GetMultiRO")]
#[macros::uow_action(entity = "Dto", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Dto", action = "GetRelationshipsFromRightIdsRO")]
impl DtoUnitOfWorkROTrait for DtoUnitOfWorkRO {}

pub struct DtoUnitOfWorkROFactory {
    context: DbContext,
}

impl DtoUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        DtoUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl DtoUnitOfWorkROFactoryTrait for DtoUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn DtoUnitOfWorkROTrait> {
        Box::new(DtoUnitOfWorkRO::new(&self.context))
    }
}
