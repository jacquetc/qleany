use std::cell::RefCell;

use anyhow::{Ok, Result};

use crate::entity::use_cases::get_entity_uc::EntityUnitOfWorkTraitRO;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::direct_access::repository_factory;
use common::entities::{EntityId, Entity};

use super::use_cases::common::EntityUnitOfWorkTrait;

pub struct EntityUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
}

impl EntityUnitOfWork {
    pub fn new(db_context: &DbContext) -> Self {
        EntityUnitOfWork {
            context: db_context.clone(),
            transaction: None,
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
        let entity = entity_repo.create(entity)?;
        Ok(entity)
    }

    fn update_entity(&self, entity: &Entity) -> Result<Entity> {
        let mut entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let entity = entity_repo.update(entity)?;
        Ok(entity)
    }

    fn delete_entity(&self, id: &EntityId) -> Result<()> {
        let mut entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        entity_repo.delete(id)?;
        Ok(())
    }
    
    fn get_relationships_of(&self, field: &common::direct_access::entity::EntityRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
        let entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = entity_repo.get_relationships_of(field, right_ids)?;
        Ok(value)
    }
    
    
    fn set_relationships(&mut self, field: &common::direct_access::entity::EntityRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<()> {
        let mut entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        entity_repo.set_relationships(field, relationships)?;
        Ok(())
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

impl EntityUnitOfWorkTraitRO for EntityUnitOfWorkRO {
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
