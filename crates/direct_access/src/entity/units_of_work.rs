use super::use_cases::common::{EntityUnitOfWorkFactoryTrait, EntityUnitOfWorkTrait};
use super::use_cases::get_entity_uc::EntityUnitOfWorkROFactoryTrait;
use crate::entity::use_cases::get_entity_uc::EntityUnitOfWorkROTrait;
use anyhow::{Ok, Result};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::direct_access::repository_factory;
use common::entities::{Entity, EntityId};
use common::event::EventHub;
use common::event::*;
use common::types;
use std::cell::RefCell;
use std::rc::Rc;

pub struct EntityUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Rc<EventHub>,
}

impl EntityUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Rc<EventHub>) -> Self {
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
        self.transaction.take().unwrap().commit()?;
        Ok(())
    }

    fn rollback(&mut self) -> Result<()> {
        self.transaction.take().unwrap().rollback()?;
        Ok(())
    }

    fn create_savepoint(&self) -> Result<()> {
        self.transaction.as_ref().unwrap().create_savepoint()?;
        Ok(())
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

impl EntityUnitOfWorkTrait for EntityUnitOfWork {
    fn get_entity(&self, id: &EntityId) -> Result<Option<Entity>> {
        let entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = entity_repo.get(id)?;
        Ok(value)
    }

    fn create_entity(&self, entity: &Entity) -> Result<Entity> {
        let mut entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let entity = entity_repo.create(&self.event_hub, entity)?;
        Ok(entity)
    }

    fn update_entity(&self, entity: &Entity) -> Result<Entity> {
        let mut entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let entity = entity_repo.update(&self.event_hub, entity)?;
        Ok(entity)
    }

    fn delete_entity(&self, id: &EntityId) -> Result<()> {
        let mut entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        entity_repo.delete(&self.event_hub, id)?;
        Ok(())
    }

    fn get_relationships_of(
        &self,
        field: &common::direct_access::entity::EntityRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
        let entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = entity_repo.get_relationships_of(field, right_ids)?;
        Ok(value)
    }

    fn set_relationships(
        &self,
        field: &common::direct_access::entity::EntityRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<()> {
        let mut entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        entity_repo.set_relationships(field, relationships)?;
        Ok(())
    }
}

pub struct EntityUnitOfWorkFactory {
    context: DbContext,
    event_hub: Rc<EventHub>,
}

impl EntityUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Rc<EventHub>) -> Self {
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

impl EntityUnitOfWorkROTrait for EntityUnitOfWorkRO {
    fn get_entity(&self, id: &EntityId) -> Result<Option<Entity>> {
        let borrowed_transaction = self.transaction.borrow();
        let entity_repo = repository_factory::read::create_entity_repository(
            &borrowed_transaction
                .as_ref()
                .expect("Transaction not started"),
        );
        let entity = entity_repo.get(id)?;
        Ok(entity)
    }
}

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
