use std::rc::Rc;

use crate::{
    database::transactions::Transaction,
    entities::{DtoField, EntityId},
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
};

use redb::Error;

pub trait DtoFieldTable {
    fn create(&mut self, dto_field: &DtoField) -> Result<DtoField, Error>;
    fn create_multi(&mut self, dto_fields: &[DtoField]) -> Result<Vec<DtoField>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<DtoField>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<DtoField>>, Error>;
    fn update(&mut self, dto_field: &DtoField) -> Result<DtoField, Error>;
    fn update_multi(&mut self, dto_fields: &[DtoField]) -> Result<Vec<DtoField>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;
}

pub trait DtoFieldTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<DtoField>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<DtoField>>, Error>;
}

pub struct DtoFieldRepository<'a> {
    redb_table: Box<dyn DtoFieldTable + 'a>,
    transaction: &'a Transaction,
}

impl<'a> DtoFieldRepository<'a> {
    pub fn new(redb_table: Box<dyn DtoFieldTable + 'a>, transaction: &'a Transaction) -> Self {
        DtoFieldRepository {
            redb_table,
            transaction,
        }
    }

    pub fn create(
        &mut self,
        event_hub: &Rc<EventHub>,
        dto_field: &DtoField,
    ) -> Result<DtoField, Error> {
        let new = self.redb_table.create(dto_field)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::DtoField(EntityEvent::Created)),
            ids: vec![new.id.clone()],
            data: None,
        });
        Ok(new)
    }

    pub fn create_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        dto_fields: &[DtoField],
    ) -> Result<Vec<DtoField>, Error> {
        let new_dto_fields = self.redb_table.create_multi(dto_fields)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::DtoField(EntityEvent::Created)),
            ids: new_dto_fields
                .iter()
                .map(|dto_field| dto_field.id.clone())
                .collect(),
            data: None,
        });

        Ok(new_dto_fields)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<DtoField>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<DtoField>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn update(
        &mut self,
        event_hub: &Rc<EventHub>,
        dto_field: &DtoField,
    ) -> Result<DtoField, Error> {
        let updated_dto_field = self.redb_table.update(dto_field)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::DtoField(EntityEvent::Updated)),
            ids: vec![updated_dto_field.id.clone()],
            data: None,
        });
        Ok(updated_dto_field)
    }

    pub fn update_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        dto_fields: &[DtoField],
    ) -> Result<Vec<DtoField>, Error> {
        let updated_dto_fields = self.redb_table.update_multi(dto_fields)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::DtoField(EntityEvent::Updated)),
            ids: updated_dto_fields
                .iter()
                .map(|dto_field| dto_field.id.clone())
                .collect(),
            data: None,
        });

        Ok(updated_dto_fields)
    }

    pub fn delete(&mut self, event_hub: &Rc<EventHub>, id: &EntityId) -> Result<(), Error> {
        let _dto_field = match self.redb_table.get(id)? {
            Some(dto_field) => dto_field,
            None => return Ok(()),
        };

        // delete dto_field
        self.redb_table.delete(id)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::DtoField(EntityEvent::Removed)),
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
        let dto_fields = self.redb_table.get_multi(ids)?;

        if dto_fields.is_empty() || dto_fields.iter().all(|root| root.is_none()) {
            return Ok(());
        }

        self.redb_table.delete_multi(ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::DtoField(EntityEvent::Removed)),
            ids: ids.into(),
            data: None,
        });

        Ok(())
    }
}

pub struct DtoFieldRepositoryRO<'a> {
    redb_table: Box<dyn DtoFieldTableRO + 'a>,
}

impl<'a> DtoFieldRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn DtoFieldTableRO + 'a>) -> Self {
        DtoFieldRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<DtoField>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<DtoField>>, Error> {
        self.redb_table.get_multi(ids)
    }
}
