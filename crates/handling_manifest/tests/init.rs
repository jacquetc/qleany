use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::database::db_context::DbContext;
use common::database::transactions::Transaction;
use common::entities::{Root, System};
use common::event::HandlingAppLifecycleEvent::InitializeApp;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use std::sync::Arc;
// Unit of work for InitializeApp

struct InitializeAppUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl InitializeAppUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        InitializeAppUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for InitializeAppUnitOfWork {
    fn begin_transaction(&mut self) -> Result<()> {
        self.transaction = Some(Transaction::begin_write_transaction(&self.context)?);
        anyhow::Ok(())
    }

    fn commit(&mut self) -> Result<()> {
        self.transaction.take().unwrap().commit()?;
        anyhow::Ok(())
    }

    fn rollback(&mut self) -> Result<()> {
        self.transaction.take().unwrap().rollback()?;
        anyhow::Ok(())
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

struct InitializeAppUnitOfWorkFactory {
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

trait InitializeAppUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn InitializeAppUnitOfWorkTrait>;
}
#[macros::uow_action(entity = "Root", action = "Create")]
#[macros::uow_action(entity = "System", action = "Create")]
trait InitializeAppUnitOfWorkTrait: CommandUnitOfWork {}

impl InitializeAppUseCase {
    fn new(uow_factory: Box<dyn InitializeAppUnitOfWorkFactoryTrait>) -> Self {
        InitializeAppUseCase { uow_factory }
    }

    fn execute(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // create system
        let system = uow.create_system(&System {
            id: 0,
            files: vec![],
        })?;

        uow.create_root(&Root {
            id: 0,
            workspace: None,
            system: Some(system.id),
        })?;

        uow.commit()?;
        Ok(())
    }
}

struct InitializeAppUseCase {
    uow_factory: Box<dyn InitializeAppUnitOfWorkFactoryTrait>,
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
