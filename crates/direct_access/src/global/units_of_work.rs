use super::use_cases::common::{GlobalUnitOfWorkFactoryTrait, GlobalUnitOfWorkTrait};
use super::use_cases::get_global_uc::GlobalUnitOfWorkROFactoryTrait;
use crate::global::use_cases::get_global_uc::GlobalUnitOfWorkROTrait;
use anyhow::{Ok, Result};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::direct_access::repository_factory;
use common::entities::{EntityId, Global};
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
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

        Ok(())
    }
}

impl GlobalUnitOfWorkTrait for GlobalUnitOfWork {
    fn get_global(&self, id: &EntityId) -> Result<Option<Global>> {
        let global_repo = repository_factory::write::create_global_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = global_repo.get(id)?;
        Ok(value)
    }

    fn create_global(&self, global: &Global) -> Result<Global> {
        let mut global_repo = repository_factory::write::create_global_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let global = global_repo.create(&self.event_hub, global)?;
        Ok(global)
    }

    fn update_global(&self, global: &Global) -> Result<Global> {
        let mut global_repo = repository_factory::write::create_global_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let global = global_repo.update(&self.event_hub, global)?;
        Ok(global)
    }

    fn delete_global(&self, id: &EntityId) -> Result<()> {
        let mut global_repo = repository_factory::write::create_global_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        global_repo.delete(&self.event_hub, id)?;
        Ok(())
    }
}

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

impl GlobalUnitOfWorkROTrait for GlobalUnitOfWorkRO {
    fn get_global(&self, id: &EntityId) -> Result<Option<Global>> {
        let borrowed_transaction = self.transaction.borrow();
        let global_repo = repository_factory::read::create_global_repository(
            &borrowed_transaction
                .as_ref()
                .expect("Transaction not started"),
        );
        let global = global_repo.get(id)?;
        Ok(global)
    }
}

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
