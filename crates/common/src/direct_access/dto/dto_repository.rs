use std::fmt::Display;
use std::sync::Arc;

use crate::{
    database::transactions::Transaction,
    direct_access::repository_factory,
    entities::Dto,
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
    types::EntityId,
};
use redb::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DtoRelationshipField {
    Fields,
}

impl Display for DtoRelationshipField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait DtoTable {
    fn create(&mut self, dto: &Dto) -> Result<Dto, Error>;
    fn create_multi(&mut self, dtos: &[Dto]) -> Result<Vec<Dto>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Dto>>, Error>;
    fn update(&mut self, dto: &Dto) -> Result<Dto, Error>;
    fn update_multi(&mut self, dtos: &[Dto]) -> Result<Vec<Dto>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;
    fn get_relationship(
        &self,
        id: &EntityId,
        field: &DtoRelationshipField,
    ) -> Result<Vec<EntityId>, Error>;
    fn get_relationships_from_right_ids(
        &self,
        field: &DtoRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn set_relationship_multi(
        &mut self,
        field: &DtoRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error>;
    fn set_relationship(
        &mut self,
        id: &EntityId,
        field: &DtoRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error>;
}

pub trait DtoTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Dto>>, Error>;
    fn get_relationship(
        &self,
        id: &EntityId,
        field: &DtoRelationshipField,
    ) -> Result<Vec<EntityId>, Error>;
    fn get_relationships_from_right_ids(
        &self,
        field: &DtoRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
}

pub struct DtoRepository<'a> {
    redb_table: Box<dyn DtoTable + 'a>,
    transaction: &'a Transaction,
}

impl<'a> DtoRepository<'a> {
    pub fn new(redb_table: Box<dyn DtoTable + 'a>, transaction: &'a Transaction) -> Self {
        DtoRepository {
            redb_table,
            transaction,
        }
    }

    pub fn create(&mut self, event_hub: &Arc<EventHub>, dto: &Dto) -> Result<Dto, Error> {
        let new = self.redb_table.create(dto)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Created)),
            ids: vec![new.id.clone()],
            data: None,
        });
        Ok(new)
    }

    pub fn create_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        dtos: &[Dto],
    ) -> Result<Vec<Dto>, Error> {
        let new_dtos = self.redb_table.create_multi(dtos)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Created)),
            ids: new_dtos.iter().map(|dto| dto.id.clone()).collect(),
            data: None,
        });

        Ok(new_dtos)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Dto>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn update(&mut self, event_hub: &Arc<EventHub>, dto: &Dto) -> Result<Dto, Error> {
        let updated_dto = self.redb_table.update(dto)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Updated)),
            ids: vec![updated_dto.id.clone()],
            data: None,
        });
        Ok(updated_dto)
    }

    pub fn update_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        dtos: &[Dto],
    ) -> Result<Vec<Dto>, Error> {
        let updated_dtos = self.redb_table.update_multi(dtos)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Updated)),
            ids: updated_dtos.iter().map(|dto| dto.id.clone()).collect(),
            data: None,
        });

        Ok(updated_dtos)
    }

    pub fn delete(&mut self, event_hub: &Arc<EventHub>, id: &EntityId) -> Result<(), Error> {
        let dto = match self.redb_table.get(id)? {
            Some(dto) => dto,
            None => return Ok(()),
        };

        // get all strong forward relationship fields
        let fields = dto.fields.clone();

        // delete all strong relationships, initiating a cascade delete
        repository_factory::write::create_dto_field_repository(self.transaction)
            .delete_multi(event_hub, &fields)?;

        // delete dto
        self.redb_table.delete(id)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Removed)),
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
        let dtos = self.redb_table.get_multi(ids)?;

        if dtos.is_empty() || dtos.iter().all(|root| root.is_none()) {
            return Ok(());
        }

        // get all strong forward relationship fields
        let mut fields: Vec<EntityId> = dtos
            .iter()
            .flat_map(|dto| dto.as_ref().map(|dto| dto.fields.clone()))
            .flatten()
            .collect();
        fields.sort();
        fields.dedup();

        // delete all strong relationships, initiating a cascade delete
        repository_factory::write::create_dto_field_repository(self.transaction)
            .delete_multi(event_hub, &fields)?;

        self.redb_table.delete_multi(ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Removed)),
            ids: ids.into(),
            data: None,
        });

        Ok(())
    }

    pub fn get_relationship(
        &self,
        id: &EntityId,
        field: &DtoRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        self.redb_table.get_relationship(id, field)
    }

    pub fn get_relationships_from_right_ids(
        &self,
        field: &DtoRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table
            .get_relationships_from_right_ids(field, right_ids)
    }

    pub fn set_relationship(
        &mut self,
        event_hub: &Arc<EventHub>,
        id: &EntityId,
        field: &DtoRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table.set_relationship(id, field, right_ids)?;

        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Updated)),
            ids: vec![id.clone()],
            data: Some(format!(
                "{}:{}",
                field,
                right_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )),
        });

        Ok(())
    }

    pub fn set_relationship_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        field: &DtoRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table
            .set_relationship_multi(field, relationships.clone())?;

        for relationship in relationships {
            let (left_id, right_ids) = relationship;
            event_hub.send_event(Event {
                origin: Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Updated)),
                ids: vec![left_id],
                data: Some(format!(
                    "{}:{}",
                    field,
                    right_ids
                        .iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )),
            });
        }

        Ok(())
    }
}

pub struct DtoRepositoryRO<'a> {
    redb_table: Box<dyn DtoTableRO + 'a>,
}

impl<'a> DtoRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn DtoTableRO + 'a>) -> Self {
        DtoRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Dto>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn get_relationship(
        &self,
        id: &EntityId,
        field: &DtoRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        self.redb_table.get_relationship(id, field)
    }

    pub fn get_relationships_from_right_ids(
        &self,
        field: &DtoRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table
            .get_relationships_from_right_ids(field, right_ids)
    }
}
