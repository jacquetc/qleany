use std::sync::Arc;

use crate::{
    database::transactions::Transaction,
    direct_access::repository_factory,
    entities::{EntityId, UseCase},
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
};

use redb::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseCaseRelationshipField {
    Entities,
    DtoIn,
    DtoOut,
}

pub trait UseCaseTable {
    fn create(&mut self, use_case: &UseCase) -> Result<UseCase, Error>;
    fn create_multi(&mut self, use_cases: &[UseCase]) -> Result<Vec<UseCase>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>, Error>;
    fn update(&mut self, use_case: &UseCase) -> Result<UseCase, Error>;
    fn update_multi(&mut self, use_cases: &[UseCase]) -> Result<Vec<UseCase>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;
    fn get_relationships_of(
        &self,
        field: &UseCaseRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn delete_all_relationships_with(
        &mut self,
        field: &UseCaseRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error>;
    fn set_relationships(
        &mut self,
        field: &UseCaseRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error>;
}

pub trait UseCaseTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>, Error>;
}

pub struct UseCaseRepository<'a> {
    redb_table: Box<dyn UseCaseTable + 'a>,
    transaction: &'a Transaction,
}

impl<'a> UseCaseRepository<'a> {
    pub fn new(redb_table: Box<dyn UseCaseTable + 'a>, transaction: &'a Transaction) -> Self {
        UseCaseRepository {
            redb_table,
            transaction,
        }
    }

    pub fn create(
        &mut self,
        event_hub: &Arc<EventHub>,
        use_case: &UseCase,
    ) -> Result<UseCase, Error> {
        let new = self.redb_table.create(use_case)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Created)),
            ids: vec![new.id.clone()],
            data: None,
        });
        Ok(new)
    }

    pub fn create_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        use_cases: &[UseCase],
    ) -> Result<Vec<UseCase>, Error> {
        let new_use_cases = self.redb_table.create_multi(use_cases)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Created)),
            ids: new_use_cases
                .iter()
                .map(|use_case| use_case.id.clone())
                .collect(),
            data: None,
        });

        Ok(new_use_cases)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn update(
        &mut self,
        event_hub: &Arc<EventHub>,
        use_case: &UseCase,
    ) -> Result<UseCase, Error> {
        let updated_use_case = self.redb_table.update(use_case)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Updated)),
            ids: vec![updated_use_case.id.clone()],
            data: None,
        });
        Ok(updated_use_case)
    }

    pub fn update_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        use_cases: &[UseCase],
    ) -> Result<Vec<UseCase>, Error> {
        let updated_use_cases = self.redb_table.update_multi(use_cases)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Updated)),
            ids: updated_use_cases
                .iter()
                .map(|use_case| use_case.id.clone())
                .collect(),
            data: None,
        });

        Ok(updated_use_cases)
    }

    pub fn delete(&mut self, event_hub: &Arc<EventHub>, id: &EntityId) -> Result<(), Error> {
        let use_case = match self.redb_table.get(id)? {
            Some(use_case) => use_case,
            None => return Ok(()),
        };

        // get all strong forward relationship fields
        let dto_in = use_case.dto_in.clone();
        let dto_out = use_case.dto_out.clone();

        // delete all strong relationships, initiating a cascade delete
        if let Some(dto_in_id) = dto_in {
            repository_factory::write::create_dto_repository(self.transaction)
                .delete(event_hub, &dto_in_id)?;
        }
        if let Some(dto_out_id) = dto_out {
            repository_factory::write::create_dto_repository(self.transaction)
                .delete(event_hub, &dto_out_id)?;
        }

        // delete use case
        self.redb_table.delete(id)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Removed)),
            ids: vec![id.clone()],
            data: None,
        });

        Ok(())
    }

    pub fn delete_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        ids: &[EntityId],
    ) -> Result<(), Error> {
        let use_cases = self.redb_table.get_multi(ids)?;

        if use_cases.is_empty() || use_cases.iter().all(|use_case| use_case.is_none()) {
            return Ok(());
        }

        // get all strong forward relationship fields
        let dto_ins: Vec<EntityId> = use_cases
            .iter()
            .filter_map(|use_case| {
                use_case
                    .as_ref()
                    .and_then(|use_case| use_case.dto_in.clone())
            })
            .collect();
        let dto_outs: Vec<EntityId> = use_cases
            .iter()
            .filter_map(|use_case| {
                use_case
                    .as_ref()
                    .and_then(|use_case| use_case.dto_out.clone())
            })
            .collect();

        // delete all strong relationships, initiating a cascade delete
        repository_factory::write::create_dto_repository(self.transaction)
            .delete_multi(event_hub, &dto_ins)?;
        repository_factory::write::create_dto_repository(self.transaction)
            .delete_multi(event_hub, &dto_outs)?;

        self.redb_table.delete_multi(ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Removed)),
            ids: ids.into(),
            data: None,
        });

        Ok(())
    }

    pub fn get_relationships_of(
        &self,
        field: &UseCaseRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table.get_relationships_of(field, right_ids)
    }

    pub fn delete_all_relationships_with(
        &mut self,
        field: &UseCaseRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table
            .delete_all_relationships_with(field, right_ids)
    }

    pub fn set_relationships(
        &mut self,
        field: &UseCaseRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table.set_relationships(field, relationships)
    }
}

pub struct UseCaseRepositoryRO<'a> {
    redb_table: Box<dyn UseCaseTableRO + 'a>,
}

impl<'a> UseCaseRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn UseCaseTableRO + 'a>) -> Self {
        UseCaseRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>, Error> {
        self.redb_table.get_multi(ids)
    }
}
