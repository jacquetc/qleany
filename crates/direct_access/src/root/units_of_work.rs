use super::use_cases::common::{
    RootUnitOfWorkFactoryTrait, RootUnitOfWorkROFactoryTrait, RootUnitOfWorkROTrait,
    RootUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::direct_access::repository_factory;
use common::entities::{EntityId, Root};
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

pub struct RootUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl RootUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        RootUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for RootUnitOfWork {
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

impl RootUnitOfWorkTrait for RootUnitOfWork {
    fn create_root(&self, root: &Root) -> Result<Root> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let root = root_repo.create(&self.event_hub, root)?;
        Ok(root)
    }

    fn create_root_multi(&self, roots: &[Root]) -> Result<Vec<Root>> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let roots = root_repo.create_multi(&self.event_hub, roots)?;
        Ok(roots)
    }

    fn get_root(&self, id: &EntityId) -> Result<Option<Root>> {
        let root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = root_repo.get(id)?;
        Ok(value)
    }

    fn get_root_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Root>>> {
        let root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = root_repo.get_multi(ids)?;
        Ok(value)
    }

    fn update_root(&self, root: &Root) -> Result<Root> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let root = root_repo.update(&self.event_hub, root)?;
        Ok(root)
    }

    fn update_root_multi(&self, roots: &[Root]) -> Result<Vec<Root>> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let roots = root_repo.update_multi(&self.event_hub, roots)?;
        Ok(roots)
    }

    fn delete_root(&self, id: &EntityId) -> Result<()> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        root_repo.delete(&self.event_hub, id)?;
        Ok(())
    }

    fn delete_root_multi(&self, ids: &[EntityId]) -> Result<()> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        root_repo.delete_multi(&self.event_hub, ids)?;
        Ok(())
    }

    fn get_relationships_of(
        &self,
        field: &common::direct_access::root::RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
        let root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = root_repo.get_relationships_of(field, right_ids)?;
        Ok(value)
    }

    fn set_relationships(
        &self,
        field: &common::direct_access::root::RootRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<()> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        root_repo.set_relationships(field, relationships)?;
        Ok(())
    }
}

pub struct RootUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl RootUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        RootUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl RootUnitOfWorkFactoryTrait for RootUnitOfWorkFactory {
    fn create(&self) -> Box<dyn RootUnitOfWorkTrait> {
        Box::new(RootUnitOfWork::new(&self.context, &self.event_hub))
    }
}

pub struct RootUnitOfWorkRO {
    context: DbContext,
    transaction: RefCell<Option<Transaction>>,
}

impl RootUnitOfWorkRO {
    pub fn new(db_context: &DbContext) -> Self {
        RootUnitOfWorkRO {
            context: db_context.clone(),
            transaction: RefCell::new(None),
        }
    }
}

impl QueryUnitOfWork for RootUnitOfWorkRO {
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

impl RootUnitOfWorkROTrait for RootUnitOfWorkRO {
    fn get_root(&self, id: &EntityId) -> Result<Option<Root>> {
        let borrowed_transaction = self.transaction.borrow();
        let root_repo = repository_factory::read::create_root_repository(
            &borrowed_transaction
                .as_ref()
                .expect("Transaction not started"),
        );
        let root = root_repo.get(id)?;
        Ok(root)
    }

    fn get_root_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Root>>> {
        let borrowed_transaction = self.transaction.borrow();
        let root_repo = repository_factory::read::create_root_repository(
            &borrowed_transaction
                .as_ref()
                .expect("Transaction not started"),
        );
        let roots = root_repo.get_multi(ids)?;
        Ok(roots)
    }
}

pub struct RootUnitOfWorkROFactory {
    context: DbContext,
}

impl RootUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        RootUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl RootUnitOfWorkROFactoryTrait for RootUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn RootUnitOfWorkROTrait> {
        Box::new(RootUnitOfWorkRO::new(&self.context))
    }
}
