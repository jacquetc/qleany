use super::use_cases::common::{
    UseCaseUnitOfWorkFactoryTrait, UseCaseUnitOfWorkROFactoryTrait, UseCaseUnitOfWorkROTrait,
    UseCaseUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::{db_context::DbContext, transactions::Transaction};
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::direct_access::repository_factory;
use common::entities::UseCase;
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

pub struct UseCaseUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl UseCaseUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        UseCaseUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
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

impl UseCaseUnitOfWorkTrait for UseCaseUnitOfWork {
    fn get_use_case(&self, id: &EntityId) -> Result<Option<UseCase>> {
        let use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = use_case_repo.get(id)?;
        Ok(value)
    }

    fn get_use_case_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>> {
        let use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = use_case_repo.get_multi(ids)?;
        Ok(value)
    }

    fn create_use_case(&self, use_case: &UseCase) -> Result<UseCase> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let use_case = use_case_repo.create(&self.event_hub, use_case)?;
        Ok(use_case)
    }

    fn create_use_case_multi(&self, use_cases: &[UseCase]) -> Result<Vec<UseCase>> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let use_cases = use_case_repo.create_multi(&self.event_hub, use_cases)?;
        Ok(use_cases)
    }

    fn update_use_case(&self, use_case: &UseCase) -> Result<UseCase> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let use_case = use_case_repo.update(&self.event_hub, use_case)?;
        Ok(use_case)
    }

    fn update_use_case_multi(&self, use_cases: &[UseCase]) -> Result<Vec<UseCase>> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let use_cases = use_case_repo.update_multi(&self.event_hub, use_cases)?;
        Ok(use_cases)
    }

    fn delete_use_case(&self, id: &EntityId) -> Result<()> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        use_case_repo.delete(&self.event_hub, id)?;
        Ok(())
    }

    fn delete_use_case_multi(&self, ids: &[EntityId]) -> Result<()> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        use_case_repo.delete_multi(&self.event_hub, ids)?;
        Ok(())
    }

    fn get_relationships_from_right_ids(
        &self,
        field: &common::direct_access::use_case::UseCaseRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
        let use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = use_case_repo.get_relationships_from_right_ids(field, right_ids)?;
        Ok(value)
    }

    fn set_relationship_multi(
        &self,
        field: &common::direct_access::use_case::UseCaseRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<()> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        use_case_repo.set_relationship_multi(&self.event_hub, field, relationships)?;
        Ok(())
    }
}

pub struct UseCaseUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl UseCaseUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        UseCaseUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl UseCaseUnitOfWorkFactoryTrait for UseCaseUnitOfWorkFactory {
    fn create(&self) -> Box<dyn UseCaseUnitOfWorkTrait> {
        Box::new(UseCaseUnitOfWork::new(&self.context, &self.event_hub))
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

impl UseCaseUnitOfWorkROTrait for UseCaseUnitOfWorkRO {
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

    fn get_use_case_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>> {
        let borrowed_transaction = self.transaction.borrow();
        let use_case_repo = repository_factory::read::create_use_case_repository(
            &borrowed_transaction
                .as_ref()
                .expect("Transaction not started"),
        );
        let use_cases = use_case_repo.get_multi(ids)?;
        Ok(use_cases)
    }
}

pub struct UseCaseUnitOfWorkROFactory {
    context: DbContext,
}

impl UseCaseUnitOfWorkROFactory {
    pub fn new(db_context: &DbContext) -> Self {
        UseCaseUnitOfWorkROFactory {
            context: db_context.clone(),
        }
    }
}

impl UseCaseUnitOfWorkROFactoryTrait for UseCaseUnitOfWorkROFactory {
    fn create(&self) -> Box<dyn UseCaseUnitOfWorkROTrait> {
        Box::new(UseCaseUnitOfWorkRO::new(&self.context))
    }
}
