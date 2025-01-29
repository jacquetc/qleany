use crate::entities::{EntityId, Root};

use super::root_table::{RootRelationshipField, RootTable, RootTableRO};
use redb::Error;

pub struct RootRepository<'a> {
    redb_table: Box<dyn RootTable + 'a>,
}

impl<'a> RootRepository<'a> {
    pub fn new(redb_table: Box<dyn RootTable + 'a>) -> Self {
        RootRepository { redb_table }
    }

    pub fn create(&mut self, root: &Root) -> Result<Root, Error> {
        self.redb_table.create(root)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Root>, Error> {
        self.redb_table.get(id)
    }

    pub fn update(&mut self, root: &Root) -> Result<Root, Error> {
        self.redb_table.update(root)
    }

    pub fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.redb_table.delete(id)
    }

    pub fn get_relationships_of(
        &self,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table.get_relationships_of(field, right_ids)
    }

    pub fn delete_all_relationships_with(
        &mut self,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table.delete_all_relationships_with(field, right_ids)
    }

    pub fn set_relationships(
        &mut self,
        field: &RootRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table.set_relationships(field, relationships)
    }

}

pub struct RootRepositoryRO<'a> {
    redb_table: Box<dyn RootTableRO + 'a>,
}

impl<'a> RootRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn RootTableRO + 'a>) -> Self {
        RootRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Root>, Error> {
        self.redb_table.get(id)
    }
}
