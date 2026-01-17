use std::sync::Arc;

use crate::{
    database::transactions::Transaction,
    entities::File,
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
    types::EntityId,
};

use redb::Error;

pub trait FileTable {
    fn create(&mut self, file: &File) -> Result<File, Error>;
    fn create_multi(&mut self, files: &[File]) -> Result<Vec<File>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<File>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<File>>, Error>;
    fn update(&mut self, file: &File) -> Result<File, Error>;
    fn update_multi(&mut self, files: &[File]) -> Result<Vec<File>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;
}

pub trait FileTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<File>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<File>>, Error>;
}

pub struct FileRepository<'a> {
    redb_table: Box<dyn FileTable + 'a>,
    _transaction: &'a Transaction,
}

impl<'a> FileRepository<'a> {
    pub fn new(redb_table: Box<dyn FileTable + 'a>, transaction: &'a Transaction) -> Self {
        FileRepository {
            redb_table,
            _transaction: transaction,
        }
    }

    pub fn create(&mut self, event_hub: &Arc<EventHub>, file: &File) -> Result<File, Error> {
        let new = self.redb_table.create(file)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::File(EntityEvent::Created)),
            ids: vec![new.id.clone()],
            data: None,
        });
        Ok(new)
    }

    pub fn create_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        files: &[File],
    ) -> Result<Vec<File>, Error> {
        let new_files = self.redb_table.create_multi(files)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::File(EntityEvent::Created)),
            ids: new_files.iter().map(|file| file.id.clone()).collect(),
            data: None,
        });

        Ok(new_files)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<File>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<File>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn update(&mut self, event_hub: &Arc<EventHub>, file: &File) -> Result<File, Error> {
        let updated_file = self.redb_table.update(file)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::File(EntityEvent::Updated)),
            ids: vec![updated_file.id.clone()],
            data: None,
        });
        Ok(updated_file)
    }

    pub fn update_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        files: &[File],
    ) -> Result<Vec<File>, Error> {
        let updated_files = self.redb_table.update_multi(files)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::File(EntityEvent::Updated)),
            ids: updated_files.iter().map(|file| file.id.clone()).collect(),
            data: None,
        });

        Ok(updated_files)
    }

    pub fn delete(&mut self, event_hub: &Arc<EventHub>, id: &EntityId) -> Result<(), Error> {
        let _file = match self.redb_table.get(id)? {
            Some(file) => file,
            None => return Ok(()),
        };

        // delete file
        self.redb_table.delete(id)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::File(EntityEvent::Removed)),
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
        let files = self.redb_table.get_multi(ids)?;

        if files.is_empty() || files.iter().all(|root| root.is_none()) {
            return Ok(());
        }

        self.redb_table.delete_multi(ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::File(EntityEvent::Removed)),
            ids: ids.into(),
            data: None,
        });

        Ok(())
    }
}

pub struct FileRepositoryRO<'a> {
    redb_table: Box<dyn FileTableRO + 'a>,
}

impl<'a> FileRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn FileTableRO + 'a>) -> Self {
        FileRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<File>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<File>>, Error> {
        self.redb_table.get_multi(ids)
    }
}
