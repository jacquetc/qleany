use std::cell::RefCell;

use anyhow::{Ok, Result};

use crate::root::use_cases::get_root_uc::RootUnitOfWorkTraitRO;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::direct_access::repository_factory;
use common::entities::{EntityId, Root};

use super::use_cases::common::RootUnitOfWorkTrait;

pub struct RootUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
}

impl RootUnitOfWork {
    pub fn new(db_context: &DbContext) -> Self {
        RootUnitOfWork {
            context: db_context.clone(),
            transaction: None,
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
}

impl RootUnitOfWorkTrait for RootUnitOfWork {
    fn get_root(&self, id: &EntityId) -> Result<Option<Root>> {
        let root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = root_repo.get(id)?;
        Ok(value)
    }

    fn create_root(&self, root: &Root) -> Result<Root> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let root = root_repo.create(root)?;
        Ok(root)
    }

    fn update_root(&self, root: &Root) -> Result<Root> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let root = root_repo.update(root)?;
        Ok(root)
    }

    fn delete_root(&self, id: &EntityId) -> Result<()> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        root_repo.delete(id)?;
        Ok(())
    }
    
    fn get_relationships_of(&self, field: &common::direct_access::root::RootRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
        let root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = root_repo.get_relationships_of(field, right_ids)?;
        Ok(value)
    }
    
    fn set_relationships(&self, field: &common::direct_access::root::RootRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<()> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        root_repo.set_relationships(field, relationships)?;
        Ok(())
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

impl RootUnitOfWorkTraitRO for RootUnitOfWorkRO {
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
}
