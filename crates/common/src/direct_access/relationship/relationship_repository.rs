use crate::entities::{EntityId, Relationship};

use super::relationship_table::{RelationshipTable, RelationshipTableRO};
use redb::Error;

pub struct RelationshipRepository<'a> {
    redb_table: Box<dyn RelationshipTable + 'a>,
}

impl<'a> RelationshipRepository<'a> {
    pub fn new(redb_table: Box<dyn RelationshipTable + 'a>) -> Self {
        RelationshipRepository { redb_table }
    }

    pub fn create(&mut self, relationship: &Relationship) -> Result<Relationship, Error> {
        self.redb_table.create(relationship)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error> {
        self.redb_table.get(id)
    }

    pub fn update(&mut self, relationship: &Relationship) -> Result<Relationship, Error> {
        self.redb_table.update(relationship)
    }

    pub fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.redb_table.delete(id)
    }
}

pub struct RelationshipRepositoryRO<'a> {
    redb_table: Box<dyn RelationshipTableRO + 'a>,
}

impl<'a> RelationshipRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn RelationshipTableRO + 'a>) -> Self {
        RelationshipRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error> {
        self.redb_table.get(id)
    }
}
