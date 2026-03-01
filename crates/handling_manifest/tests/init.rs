use std::cell::RefCell;
use anyhow::Result;
use common::database::CommandUnitOfWork;
use common::database::db_context::DbContext;
use common::database::transactions::Transaction;
use common::entities::{Root, System};
use common::event::HandlingAppLifecycleEvent::InitializeApp;
use common::event::{AllEvent, DirectAccessEntity, Event, EventBuffer, EventHub, Origin};
use common::types;
use std::sync::Arc;
// Unit of work for InitializeApp

pub struct InitializeAppUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
    event_buffer: RefCell<EventBuffer>,
}

impl InitializeAppUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        InitializeAppUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
            event_buffer: RefCell::new(EventBuffer::new()),
        }
    }
}

impl CommandUnitOfWork for InitializeAppUnitOfWork {
    fn begin_transaction(&mut self) -> Result<()> {
        self.transaction = Some(Transaction::begin_write_transaction(&self.context)?);
        self.event_buffer.get_mut().begin_buffering();
        anyhow::Ok(())
    }

    fn commit(&mut self) -> Result<()> {
        self.transaction.take().unwrap().commit()?;
        for event in self.event_buffer.get_mut().flush() {
            self.event_hub.send_event(event);
        }
        anyhow::Ok(())
    }

    fn rollback(&mut self) -> Result<()> {
        self.transaction.take().unwrap().rollback()?;
        self.event_buffer.get_mut().discard();
        anyhow::Ok(())
    }

    fn create_savepoint(&self) -> Result<types::Savepoint> {
        self.transaction.as_ref().unwrap().create_savepoint()
    }

    fn restore_to_savepoint(&mut self, savepoint: types::Savepoint) -> Result<()> {
        let mut transaction = self.transaction.take().unwrap();
        transaction.restore_to_savepoint(savepoint)?;

        // Discard buffered events — savepoint restore invalidated them
        self.event_buffer.get_mut().discard();

        // Send Reset immediately (not buffered — UI must refresh now)
        self.event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::All(AllEvent::Reset)),
            ids: vec![],
            data: None,
        });

        // Recreate the transaction after restoring to savepoint
        self.transaction = Some(transaction);

        anyhow::Ok(())
    }
}

//TODO: adapt entities and actions to real use : Create, CreateMulti, Get, GetMulti, Update, UpdateMulti, Delete,
//DeleteMulti, GetRO, GetMultiRO, GetRelationship, GetRelationshipRO, GetRelationshipsFromRightIds,
//GetRelationshipsFromRightIdsRO, SetRelationship, SetRelationshipMulti
//
// You have here a read-write unit of work.
//
// RO means Read Only, so *RO actions should not used be here.
// Don't forget to set thread_safe = true for long operation's unit of work.
// Do not mix read-only and write actions in the same unit of work.
//
// Exactly the same macros must be set in the use case uow trait file in ../use_cases/initialize_app_uc.rs
//
#[macros::uow_action(entity = "Root", action = "Create")]
#[macros::uow_action(entity = "System", action = "Create")]
impl InitializeAppUnitOfWorkTrait for InitializeAppUnitOfWork {}

pub struct InitializeAppUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl InitializeAppUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        InitializeAppUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl InitializeAppUnitOfWorkFactoryTrait for InitializeAppUnitOfWorkFactory {
    fn create(&self) -> Box<dyn InitializeAppUnitOfWorkTrait> {
        Box::new(InitializeAppUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub trait InitializeAppUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn InitializeAppUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "Create")]
#[macros::uow_action(entity = "System", action = "Create")]
pub trait InitializeAppUnitOfWorkTrait: CommandUnitOfWork {}

pub struct InitializeAppUseCase {
    uow_factory: Box<dyn InitializeAppUnitOfWorkFactoryTrait>,
}

impl InitializeAppUseCase {
    pub fn new(uow_factory: Box<dyn InitializeAppUnitOfWorkFactoryTrait>) -> Self {
        InitializeAppUseCase { uow_factory }
    }

    pub fn execute(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // create system
        let system = uow.create_system(&System {
            id: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            files: vec![],
        })?;

        uow.create_root(&Root {
            id: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            workspace: None,
            system: Some(system.id),
        })?;

        uow.commit()?;
        Ok(())
    }
}

pub fn initialize_app(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Result<()> {
    let uow_context = InitializeAppUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut uc = InitializeAppUseCase::new(Box::new(uow_context));
    let return_dto = uc.execute()?;
    // Notify that the handling manifest has been loaded
    event_hub.send_event(Event {
        origin: Origin::HandlingAppLifecycle(InitializeApp),
        ids: vec![],
        data: None,
    });
    Ok(return_dto)
}
