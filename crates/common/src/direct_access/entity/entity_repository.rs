use crate::entities::{EntityId, Entity};

use super::entity_table::{EntityRelationshipField, EntityTable, EntityTableRO};
use redb::Error;

pub struct EntityRepository<'a> {
    redb_table: Box<dyn EntityTable + 'a>,
}

impl<'a> EntityRepository<'a> {
    pub fn new(redb_table: Box<dyn EntityTable + 'a>) -> Self {
        EntityRepository { redb_table }
    }

    pub fn create(&mut self, entity: &Entity) -> Result<Entity, Error> {
        self.redb_table.create(entity)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error> {
        self.redb_table.get(id)
    }

    pub fn update(&mut self, entity: &Entity) -> Result<Entity, Error> {
        self.redb_table.update(entity)
    }

    pub fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.redb_table.delete(id)
    }

    
    pub fn get_relationships_of(
        &self,
        field: &EntityRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table.get_relationships_of(field, right_ids)
    }

    pub fn delete_all_relationships_with(
        &mut self,
        field: &EntityRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table.delete_all_relationships_with(field, right_ids)
    }

    pub fn set_relationships(
        &mut self,
        field: &EntityRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table.set_relationships(field, relationships)
    }

}

pub struct EntityRepositoryRO<'a> {
    redb_table: Box<dyn EntityTableRO + 'a>,
}

impl<'a> EntityRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn EntityTableRO + 'a>) -> Self {
        EntityRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error> {
        self.redb_table.get(id)
    }
}
