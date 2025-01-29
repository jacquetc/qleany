use crate::entities::{EntityId, Dto};

use super::{dto_table::{DtoTable, DtoTableRO}, DtoRelationshipField};
use redb::Error;

pub struct DtoRepository<'a> {
    redb_table: Box<dyn DtoTable + 'a>,
}

impl<'a> DtoRepository<'a> {
    pub fn new(redb_table: Box<dyn DtoTable + 'a>) -> Self {
        DtoRepository { redb_table }
    }

    pub fn create(&mut self, dto: &Dto) -> Result<Dto, Error> {
        self.redb_table.create(dto)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error> {
        self.redb_table.get(id)
    }

    pub fn update(&mut self, dto: &Dto) -> Result<Dto, Error> {
        self.redb_table.update(dto)
    }

    pub fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.redb_table.delete(id)
    }
    
    pub fn get_relationships_of(
        &self,
        field: &DtoRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table.get_relationships_of(field, right_ids)
    }

    pub fn delete_all_relationships_with(
        &mut self,
        field: &DtoRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table.delete_all_relationships_with(field, right_ids)
    }

    pub fn set_relationships(
        &mut self,
        field: &DtoRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table.set_relationships(field, relationships)
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
}
