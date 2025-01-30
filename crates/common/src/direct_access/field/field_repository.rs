use std::rc::Rc;

use crate::{
    database::transactions::Transaction,
    entities::{EntityId, Field},
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
};

use redb::Error;

pub enum FieldRelationshipField {
    Entity,
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
    fn get_relationships_of(
        &self,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn delete_all_relationships_with(
        &mut self,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error>;
    fn set_relationships(
        &mut self,
        field: &FieldRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error>;
}

pub trait FieldTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Field>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Field>>, Error>;
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

    pub fn create(&mut self, event_hub: &Rc<EventHub>, field: &Field) -> Result<Field, Error> {
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
        event_hub: &Rc<EventHub>,
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

    pub fn update(&mut self, event_hub: &Rc<EventHub>, field: &Field) -> Result<Field, Error> {
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
        event_hub: &Rc<EventHub>,
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

    pub fn delete(&mut self, event_hub: &Rc<EventHub>, id: &EntityId) -> Result<(), Error> {
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
        event_hub: &Rc<EventHub>,
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

    pub fn get_relationships_of(
        &self,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table.get_relationships_of(field, right_ids)
    }

    pub fn delete_all_relationships_with(
        &mut self,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table
            .delete_all_relationships_with(field, right_ids)
    }

    pub fn set_relationships(
        &mut self,
        field: &FieldRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table.set_relationships(field, relationships)
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
}
