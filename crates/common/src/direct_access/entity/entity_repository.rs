use std::rc::Rc;

use crate::{
    database::transactions::Transaction,
    direct_access::repository_factory,
    entities::{Entity, EntityId},
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
};

use redb::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityRelationshipField {
    Fields,
    Relationships,
}

pub trait EntityTable {
    fn create(&mut self, entity: &Entity) -> Result<Entity, Error>;
    fn create_multi(&mut self, entities: &[Entity]) -> Result<Vec<Entity>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Entity>>, Error>;
    fn update(&mut self, entity: &Entity) -> Result<Entity, Error>;
    fn update_multi(&mut self, entities: &[Entity]) -> Result<Vec<Entity>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;
    fn get_relationships_of(
        &self,
        field: &EntityRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn delete_all_relationships_with(
        &mut self,
        field: &EntityRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error>;
    fn set_relationships(
        &mut self,
        field: &EntityRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error>;
}

pub trait EntityTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Entity>>, Error>;
}

pub struct EntityRepository<'a> {
    redb_table: Box<dyn EntityTable + 'a>,
    transaction: &'a Transaction,
}

impl<'a> EntityRepository<'a> {
    pub fn new(redb_table: Box<dyn EntityTable + 'a>, transaction: &'a Transaction) -> Self {
        EntityRepository {
            redb_table,
            transaction,
        }
    }

    pub fn create(&mut self, event_hub: &Rc<EventHub>, entity: &Entity) -> Result<Entity, Error> {
        let new = self.redb_table.create(entity)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Entity(EntityEvent::Created)),
            ids: vec![new.id.clone()],
            data: None,
        });
        Ok(new)
    }

    pub fn create_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        entities: &[Entity],
    ) -> Result<Vec<Entity>, Error> {
        let new_entities = self.redb_table.create_multi(entities)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Entity(EntityEvent::Created)),
            ids: new_entities
                .iter()
                .map(|entity| entity.id.clone())
                .collect(),
            data: None,
        });

        Ok(new_entities)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Entity>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn update(&mut self, event_hub: &Rc<EventHub>, entity: &Entity) -> Result<Entity, Error> {
        let updated_entity = self.redb_table.update(entity)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Entity(EntityEvent::Updated)),
            ids: vec![updated_entity.id.clone()],
            data: None,
        });
        Ok(updated_entity)
    }

    pub fn update_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        entities: &[Entity],
    ) -> Result<Vec<Entity>, Error> {
        let updated_entities = self.redb_table.update_multi(entities)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Entity(EntityEvent::Updated)),
            ids: updated_entities
                .iter()
                .map(|entity| entity.id.clone())
                .collect(),
            data: None,
        });

        Ok(updated_entities)
    }

    pub fn delete(&mut self, event_hub: &Rc<EventHub>, id: &EntityId) -> Result<(), Error> {
        let entity = match self.redb_table.get(id)? {
            Some(entity) => entity,
            None => return Ok(()),
        };

        // get all strong forward relationship fields
        let fields = entity.fields.clone();
        let relationships = entity.relationships.clone();

        // delete all strong relationships, initiating a cascade delete
        repository_factory::write::create_field_repository(self.transaction)
            .delete_multi(event_hub, &fields)?;
        repository_factory::write::create_relationship_repository(self.transaction)
            .delete_multi(event_hub, &relationships)?;

        // delete entity
        self.redb_table.delete(id)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Entity(EntityEvent::Removed)),
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
        let entities = self.redb_table.get_multi(ids)?;

        if entities.is_empty() || entities.iter().all(|root| root.is_none()) {
            return Ok(());
        }

        // get all strong forward relationship fields
        let fields: Vec<EntityId> = entities
            .iter()
            .flat_map(|entity| entity.as_ref().map(|entity| entity.fields.clone()))
            .flatten()
            .collect();
        let relationships: Vec<EntityId> = entities
            .iter()
            .flat_map(|entity| entity.as_ref().map(|entity| entity.relationships.clone()))
            .flatten()
            .collect();

        // delete all strong relationships, initiating a cascade delete
        repository_factory::write::create_field_repository(self.transaction)
            .delete_multi(event_hub, &fields)?;
        repository_factory::write::create_relationship_repository(self.transaction)
            .delete_multi(event_hub, &relationships)?;

        self.redb_table.delete_multi(ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Entity(EntityEvent::Removed)),
            ids: ids.into(),
            data: None,
        });

        Ok(())
    }

    pub fn get_relationships_of(
        &self,
        field: &EntityRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table.get_relationships_of(field, right_ids)
    }

    pub fn delete_all_relationships_with(
        &mut self,
        field: &EntityRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table
            .delete_all_relationships_with(field, right_ids)
    }

    pub fn set_relationships(
        &mut self,
        field: &EntityRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table.set_relationships(field, relationships)
    }
}

pub struct EntityRepositoryRO<'a> {
    redb_table: Box<dyn EntityTableRO + 'a>,
}

impl<'a> EntityRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn EntityTableRO + 'a>) -> Self {
        EntityRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Entity>>, Error> {
        self.redb_table.get_multi(ids)
    }
}
