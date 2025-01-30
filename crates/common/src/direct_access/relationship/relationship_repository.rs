use std::rc::Rc;

use crate::{
    database::transactions::Transaction,
    entities::{Cardinality, Direction, EntityId, Order, Relationship, RelationshipType, Strength},
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
};

use redb::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationshipRelationshipField {
    LeftEntity,
    RightEntity,
}

pub trait RelationshipTable {
    fn create(&mut self, relationship: &Relationship) -> Result<Relationship, Error>;
    fn create_multi(&mut self, relationships: &[Relationship]) -> Result<Vec<Relationship>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Relationship>>, Error>;
    fn update(&mut self, relationship: &Relationship) -> Result<Relationship, Error>;
    fn update_multi(&mut self, relationships: &[Relationship]) -> Result<Vec<Relationship>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;
}

pub trait RelationshipTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Relationship>>, Error>;
}

pub struct RelationshipRepository<'a> {
    redb_table: Box<dyn RelationshipTable + 'a>,
    transaction: &'a Transaction,
}

impl<'a> RelationshipRepository<'a> {
    pub fn new(redb_table: Box<dyn RelationshipTable + 'a>, transaction: &'a Transaction) -> Self {
        RelationshipRepository {
            redb_table,
            transaction,
        }
    }

    pub fn create(
        &mut self,
        event_hub: &Rc<EventHub>,
        relationship: &Relationship,
    ) -> Result<Relationship, Error> {
        let new = self.redb_table.create(relationship)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Relationship(EntityEvent::Created)),
            ids: vec![new.id.clone()],
            data: None,
        });
        Ok(new)
    }

    pub fn create_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        relationships: &[Relationship],
    ) -> Result<Vec<Relationship>, Error> {
        let new_relationships = self.redb_table.create_multi(relationships)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Relationship(EntityEvent::Created)),
            ids: new_relationships
                .iter()
                .map(|relationship| relationship.id.clone())
                .collect(),
            data: None,
        });

        Ok(new_relationships)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Relationship>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn update(
        &mut self,
        event_hub: &Rc<EventHub>,
        relationship: &Relationship,
    ) -> Result<Relationship, Error> {
        let updated_relationship = self.redb_table.update(relationship)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Relationship(EntityEvent::Updated)),
            ids: vec![updated_relationship.id.clone()],
            data: None,
        });
        Ok(updated_relationship)
    }

    pub fn update_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        relationships: &[Relationship],
    ) -> Result<Vec<Relationship>, Error> {
        let updated_relationships = self.redb_table.update_multi(relationships)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Relationship(EntityEvent::Updated)),
            ids: updated_relationships
                .iter()
                .map(|relationship| relationship.id.clone())
                .collect(),
            data: None,
        });

        Ok(updated_relationships)
    }

    pub fn delete(&mut self, event_hub: &Rc<EventHub>, id: &EntityId) -> Result<(), Error> {
        let _relationship = match self.redb_table.get(id)? {
            Some(relationship) => relationship,
            None => return Ok(()),
        };

        // delete relationship
        self.redb_table.delete(id)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Relationship(EntityEvent::Removed)),
            ids: vec![id.clone()],
            data: None,
        });

        Ok(())
    }

    pub fn delete_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        ids: &[EntityId],
    ) -> Result<(), Error> {
        let relationships = self.redb_table.get_multi(ids)?;

        if relationships.is_empty() || relationships.iter().all(|root| root.is_none()) {
            return Ok(());
        }

        self.redb_table.delete_multi(ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Relationship(EntityEvent::Removed)),
            ids: ids.into(),
            data: None,
        });

        Ok(())
    }
}

pub struct RelationshipRepositoryRO<'a> {
    redb_table: Box<dyn RelationshipTableRO + 'a>,
}

impl<'a> RelationshipRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn RelationshipTableRO + 'a>) -> Self {
        RelationshipRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Relationship>>, Error> {
        self.redb_table.get_multi(ids)
    }
}
