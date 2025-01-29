use crate::entities::{EntityId, Global};
use super::global_table::{GlobalTable, GlobalTableRO};
use redb::Error;

pub struct GlobalRepository<'a> {
    redb_table: Box<dyn GlobalTable + 'a>,
}

impl<'a> GlobalRepository<'a> {
    pub fn new(redb_table: Box<dyn GlobalTable + 'a>) -> Self {
        GlobalRepository { redb_table }
    }

    pub fn create(&mut self, global: &Global) -> Result<Global, Error> {
        self.redb_table.create(global)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Global>, Error> {
        self.redb_table.get(id)
    }

    pub fn update(&mut self, global: &Global) -> Result<Global, Error> {
        self.redb_table.update(global)
    }

    pub fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.redb_table.delete(id)
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
}
