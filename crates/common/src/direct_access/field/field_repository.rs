use crate::entities::{EntityId, Field};

use super::field_table::{FieldRelationshipField, FieldTable, FieldTableRO};
use redb::Error;

pub struct FieldRepository<'a> {
    redb_table: Box<dyn FieldTable + 'a>,
}

impl<'a> FieldRepository<'a> {
    pub fn new(redb_table: Box<dyn FieldTable + 'a>) -> Self {
        FieldRepository { redb_table }
    }

    pub fn create(&mut self, field: &Field) -> Result<Field, Error> {
        self.redb_table.create(field)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Field>, Error> {
        self.redb_table.get(id)
    }

    pub fn update(&mut self, field: &Field) -> Result<Field, Error> {
        self.redb_table.update(field)
    }

    pub fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.redb_table.delete(id)
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
        self.redb_table.delete_all_relationships_with(field, right_ids)
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
}
