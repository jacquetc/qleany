use std::cell::RefCell;

use anyhow::{Ok, Result};

use crate::use_case::use_cases::get_use_case_uc::UseCaseUnitOfWorkTraitRO;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::direct_access::repository_factory;
use common::entities::{EntityId, UseCase};

use super::use_cases::common::UseCaseUnitOfWorkTrait;

pub struct UseCaseUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
}

impl UseCaseUnitOfWork {
    pub fn new(db_context: &DbContext) -> Self {
        UseCaseUnitOfWork {
            context: db_context.clone(),
            transaction: None,
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
}

impl UseCaseUnitOfWorkTrait for UseCaseUnitOfWork {
    fn get_use_case(&self, id: &EntityId) -> Result<Option<UseCase>> {
        let use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = use_case_repo.get(id)?;
        Ok(value)
    }

    fn create_use_case(&self, use_case: &UseCase) -> Result<UseCase> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let use_case = use_case_repo.create(use_case)?;
        Ok(use_case)
    }

    fn update_use_case(&self, use_case: &UseCase) -> Result<UseCase> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let use_case = use_case_repo.update(use_case)?;
        Ok(use_case)
    }

    fn delete_use_case(&self, id: &EntityId) -> Result<()> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        use_case_repo.delete(id)?;
        Ok(())
    }
    
    fn get_relationships_of(&self, field: &common::direct_access::use_case::UseCaseRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
        let use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = use_case_repo.get_relationships_of(field, right_ids)?;
        Ok(value)
    }
    
    fn set_relationships(&mut self, field: &common::direct_access::use_case::UseCaseRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<()> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        use_case_repo.set_relationships(field, relationships)?;
        Ok(())
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

impl UseCaseUnitOfWorkTraitRO for UseCaseUnitOfWorkRO {
    fn get_use_case(&self, id: &EntityId) -> Result<Option<UseCase>> {
        let borrowed_transaction = self.transaction.borrow();
        let use_case_repo = repository_factory::read::create_use_case_repository(
            &borrowed_transaction
                .as_ref()
                .expect("Transaction not started"),
        );
        let use_case = use_case_repo.get(id)?;
        Ok(use_case)
    }
}
