use crate::entities::{EntityId, DtoField};

use super::dto_field_table::{DtoFieldTable, DtoFieldTableRO};
use redb::Error;

pub struct DtoFieldRepository<'a> {
    redb_table: Box<dyn DtoFieldTable + 'a>,
}

impl<'a> DtoFieldRepository<'a> {
    pub fn new(redb_table: Box<dyn DtoFieldTable + 'a>) -> Self {
        DtoFieldRepository { redb_table }
    }

    pub fn create(&mut self, dto_field: &DtoField) -> Result<DtoField, Error> {
        self.redb_table.create(dto_field)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<DtoField>, Error> {
        self.redb_table.get(id)
    }

    pub fn update(&mut self, dto_field: &DtoField) -> Result<DtoField, Error> {
        self.redb_table.update(dto_field)
    }

    pub fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.redb_table.delete(id)
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
}
