use std::rc::Rc;

use crate::{
    database::transactions::Transaction,
    entities::{EntityId, Global},
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
};

use redb::Error;

pub trait GlobalTable {
    fn create(&mut self, global: &Global) -> Result<Global, Error>;
    fn create_multi(&mut self, globals: &[Global]) -> Result<Vec<Global>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Global>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Global>>, Error>;
    fn update(&mut self, global: &Global) -> Result<Global, Error>;
    fn update_multi(&mut self, globals: &[Global]) -> Result<Vec<Global>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;
}

pub trait GlobalTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Global>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Global>>, Error>;
}

pub struct GlobalRepository<'a> {
    redb_table: Box<dyn GlobalTable + 'a>,
    transaction: &'a Transaction,
}

impl<'a> GlobalRepository<'a> {
    pub fn new(redb_table: Box<dyn GlobalTable + 'a>, transaction: &'a Transaction) -> Self {
        GlobalRepository {
            redb_table,
            transaction,
        }
    }

    pub fn create(&mut self, event_hub: &Rc<EventHub>, global: &Global) -> Result<Global, Error> {
        let new = self.redb_table.create(global)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Global(EntityEvent::Created)),
            ids: vec![new.id.clone()],
            data: None,
        });
        Ok(new)
    }

    pub fn create_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        globals: &[Global],
    ) -> Result<Vec<Global>, Error> {
        let new_globals = self.redb_table.create_multi(globals)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Global(EntityEvent::Created)),
            ids: new_globals.iter().map(|global| global.id.clone()).collect(),
            data: None,
        });

        Ok(new_globals)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Global>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Global>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn update(&mut self, event_hub: &Rc<EventHub>, global: &Global) -> Result<Global, Error> {
        let updated_global = self.redb_table.update(global)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Global(EntityEvent::Updated)),
            ids: vec![updated_global.id.clone()],
            data: None,
        });
        Ok(updated_global)
    }

    pub fn update_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        globals: &[Global],
    ) -> Result<Vec<Global>, Error> {
        let updated_globals = self.redb_table.update_multi(globals)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Global(EntityEvent::Updated)),
            ids: updated_globals
                .iter()
                .map(|global| global.id.clone())
                .collect(),
            data: None,
        });

        Ok(updated_globals)
    }

    pub fn delete(&mut self, event_hub: &Rc<EventHub>, id: &EntityId) -> Result<(), Error> {
        let _global = match self.redb_table.get(id)? {
            Some(global) => global,
            None => return Ok(()),
        };

        // delete global
        self.redb_table.delete(id)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Global(EntityEvent::Removed)),
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
            origin: Origin::DirectAccess(DirectAccessEntity::Global(EntityEvent::Removed)),
            ids: ids.into(),
            data: None,
        });

        Ok(())
    }
}

pub struct GlobalRepositoryRO<'a> {
    redb_table: Box<dyn GlobalTableRO + 'a>,
}

impl<'a> GlobalRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn GlobalTableRO + 'a>) -> Self {
        GlobalRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Global>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Global>>, Error> {
        self.redb_table.get_multi(ids)
    }
}
