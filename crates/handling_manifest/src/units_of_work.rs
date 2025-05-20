use crate::use_cases::load_uc::{LoadUnitOfWorkFactoryTrait, LoadUnitOfWorkTrait};
use anyhow::{Ok, Result};
use common::database::CommandUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::direct_access::repository_factory;
use common::entities::{
    Dto, DtoField, Entity, Feature, Field, Global, Relationship, Root, UseCase,
};
use common::event::{AllEvent, DirectAccessEntity, Event, EventHub, Origin};
use common::types;
use common::types::EntityId;
use std::sync::Arc;

pub struct LoadUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
}

impl LoadUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        LoadUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
        }
    }
}

impl CommandUnitOfWork for LoadUnitOfWork {
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

impl LoadUnitOfWorkTrait for LoadUnitOfWork {
    fn create_root(&self, root: &Root) -> Result<Root> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let root = root_repo.create(&self.event_hub, root)?;
        Ok(root)
    }

    fn get_root(&self, id: &EntityId) -> Result<Option<Root>> {
        let root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = root_repo.get(id)?;
        Ok(value)
    }

    fn update_root(&self, root: &Root) -> Result<Root> {
        let mut root_repo = repository_factory::write::create_root_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let root = root_repo.update(&self.event_hub, root)?;
        Ok(root)
    }

    fn create_global(&self, global: &Global) -> Result<Global> {
        let mut global_repo = repository_factory::write::create_global_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let global = global_repo.create(&self.event_hub, global)?;
        Ok(global)
    }

    fn create_feature(&self, feature: &Feature) -> Result<Feature> {
        let mut feature_repo = repository_factory::write::create_feature_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let feature = feature_repo.create(&self.event_hub, feature)?;
        Ok(feature)
    }

    fn create_use_case(&self, use_case: &UseCase) -> Result<UseCase> {
        let mut use_case_repo = repository_factory::write::create_use_case_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let use_case = use_case_repo.create(&self.event_hub, use_case)?;
        Ok(use_case)
    }

    fn create_entity(&self, entity: &Entity) -> Result<Entity> {
        let mut entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let entity = entity_repo.create(&self.event_hub, entity)?;
        Ok(entity)
    }

    fn get_entity(&self, id: &EntityId) -> Result<Option<Entity>> {
        let entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = entity_repo.get(id)?;
        Ok(value)
    }

    fn update_entity(&self, entity: &Entity) -> Result<Entity> {
        let mut entity_repo = repository_factory::write::create_entity_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let entity = entity_repo.update(&self.event_hub, entity)?;
        Ok(entity)
    }

    fn create_field(&self, field: &Field) -> Result<Field> {
        let mut field_repo = repository_factory::write::create_field_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let field = field_repo.create(&self.event_hub, field)?;
        Ok(field)
    }

    fn get_fields(&self, ids: &[EntityId]) -> Result<Vec<Option<Field>>> {
        let field_repo = repository_factory::write::create_field_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let value = field_repo.get_multi(ids)?;
        Ok(value)
    }

    fn create_dto(&self, dto: &Dto) -> Result<Dto> {
        let mut dto_repo = repository_factory::write::create_dto_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let dto = dto_repo.create(&self.event_hub, dto)?;
        Ok(dto)
    }

    fn create_dto_field(&self, dto_field: &DtoField) -> Result<DtoField> {
        let mut dto_field_repo = repository_factory::write::create_dto_field_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let dto_field = dto_field_repo.create(&self.event_hub, dto_field)?;
        Ok(dto_field)
    }

    fn create_relationships(&self, relationships: &[Relationship]) -> Result<Vec<Relationship>> {
        let mut relationship_repo = repository_factory::write::create_relationship_repository(
            &self.transaction.as_ref().expect("Transaction not started"),
        );
        let relationships = relationship_repo.create_multi(&self.event_hub, relationships)?;
        Ok(relationships)
    }
}

pub struct LoadUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl LoadUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        LoadUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl LoadUnitOfWorkFactoryTrait for LoadUnitOfWorkFactory {
    fn create(&self) -> Box<dyn LoadUnitOfWorkTrait> {
        Box::new(LoadUnitOfWork::new(&self.context, &self.event_hub))
    }
}
