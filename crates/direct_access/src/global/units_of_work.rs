use std::cell::RefCell;

use anyhow::{Ok, Result};

use crate::global::use_cases::get_global_uc::GlobalUnitOfWorkTraitRO;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::direct_access::repository_factory;
use common::entities::{EntityId, Global};

use super::use_cases::common::GlobalUnitOfWorkTrait;

pub struct GlobalUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
}

impl GlobalUnitOfWork {
    pub fn new(db_context: &DbContext) -> Self {
        GlobalUnitOfWork {
            context: db_context.clone(),
            transaction: None,
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
        let global = global_repo.create(global)?;
        Ok(global)
    }

    fn update_global(&self, global: &Global) -> Result<Global> {
        let mut global_repo = repository_factory::write::create_global_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let global = global_repo.update(global)?;
        Ok(global)
    }

    fn delete_global(&self, id: &EntityId) -> Result<()> {
        let mut global_repo = repository_factory::write::create_global_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        global_repo.delete(id)?;
        Ok(())
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

impl GlobalUnitOfWorkTraitRO for GlobalUnitOfWorkRO {
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
