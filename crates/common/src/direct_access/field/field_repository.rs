use std::fmt::Display;
use std::sync::Arc;

use crate::{
    database::transactions::Transaction,
    entities::Field,
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
    types::EntityId,
};
use redb::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldRelationshipField {
    Entity,
}

impl Display for FieldRelationshipField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait FieldTable {
    fn create(&mut self, field: &Field) -> Result<Field, Error>;
    fn create_multi(&mut self, fields: &[Field]) -> Result<Vec<Field>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Field>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Field>>, Error>;
    fn update(&mut self, field: &Field) -> Result<Field, Error>;
    fn update_multi(&mut self, fields: &[Field]) -> Result<Vec<Field>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;
    fn get_relationship(
        &self,
        id: &EntityId,
        field: &FieldRelationshipField,
    ) -> Result<Vec<EntityId>, Error>;
    fn get_relationships_from_right_ids(
        &self,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn set_relationship_multi(
        &mut self,
        field: &FieldRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error>;
    fn set_relationship(
        &mut self,
        id: &EntityId,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error>;
}

pub trait FieldTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Field>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Field>>, Error>;
    fn get_relationship(
        &self,
        id: &EntityId,
        field: &FieldRelationshipField,
    ) -> Result<Vec<EntityId>, Error>;
    fn get_relationships_from_right_ids(
        &self,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
}

pub struct FieldRepository<'a> {
    redb_table: Box<dyn FieldTable + 'a>,
    transaction: &'a Transaction,
}

impl<'a> FieldRepository<'a> {
    pub fn new(redb_table: Box<dyn FieldTable + 'a>, transaction: &'a Transaction) -> Self {
        FieldRepository {
            redb_table,
            transaction,
        }
    }

    pub fn create(&mut self, event_hub: &Arc<EventHub>, field: &Field) -> Result<Field, Error> {
        let new = self.redb_table.create(field)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Field(EntityEvent::Created)),
            ids: vec![new.id.clone()],
            data: None,
        });
        Ok(new)
    }

    pub fn create_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        fields: &[Field],
    ) -> Result<Vec<Field>, Error> {
        let new_fields = self.redb_table.create_multi(fields)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Field(EntityEvent::Created)),
            ids: new_fields.iter().map(|field| field.id.clone()).collect(),
            data: None,
        });

        Ok(new_fields)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Field>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Field>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn update(&mut self, event_hub: &Arc<EventHub>, field: &Field) -> Result<Field, Error> {
        let updated_field = self.redb_table.update(field)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Field(EntityEvent::Updated)),
            ids: vec![updated_field.id.clone()],
            data: None,
        });
        Ok(updated_field)
    }

    pub fn update_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        fields: &[Field],
    ) -> Result<Vec<Field>, Error> {
        let updated_fields = self.redb_table.update_multi(fields)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Field(EntityEvent::Updated)),
            ids: updated_fields
                .iter()
                .map(|field| field.id.clone())
                .collect(),
            data: None,
        });

        Ok(updated_fields)
    }

    pub fn delete(&mut self, event_hub: &Arc<EventHub>, id: &EntityId) -> Result<(), Error> {
        let _field = match self.redb_table.get(id)? {
            Some(field) => field,
            None => return Ok(()),
        };

        // delete field
        self.redb_table.delete(id)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Field(EntityEvent::Removed)),
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
        let globals = self.redb_table.get_multi(ids)?;

        if globals.is_empty() || globals.iter().all(|root| root.is_none()) {
            return Ok(());
        }

        self.redb_table.delete_multi(ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Field(EntityEvent::Removed)),
            ids: ids.into(),
            data: None,
        });

        Ok(())
    }
    pub fn get_relationship(
        &self,
        id: &EntityId,
        field: &FieldRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        self.redb_table.get_relationship(id, field)
    }
    pub fn get_relationships_from_right_ids(
        &self,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table
            .get_relationships_from_right_ids(field, right_ids)
    }

    pub fn set_relationship_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        field: &FieldRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table
            .set_relationship_multi(field, relationships.clone())?;

        for relationship in relationships {
            let (left_id, right_ids) = relationship;
            event_hub.send_event(Event {
                origin: Origin::DirectAccess(DirectAccessEntity::Relationship(
                    EntityEvent::Updated,
                )),
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

    pub fn set_relationship(
        &mut self,
        event_hub: &Arc<EventHub>,
        id: &EntityId,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table.set_relationship(id, field, right_ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Relationship(EntityEvent::Updated)),
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
}

pub struct FieldRepositoryRO<'a> {
    redb_table: Box<dyn FieldTableRO + 'a>,
}

impl<'a> FieldRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn FieldTableRO + 'a>) -> Self {
        FieldRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Field>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Field>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn get_relationship(
        &self,
        id: &EntityId,
        field: &FieldRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        self.redb_table.get_relationship(id, field)
    }
    pub fn get_relationships_from_right_ids(
        &self,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table
            .get_relationships_from_right_ids(field, right_ids)
    }
}
