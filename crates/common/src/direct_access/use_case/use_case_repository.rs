use crate::entities::{EntityId, UseCase};

use super::use_case_table::{UseCaseRelationshipField, UseCaseTable, UseCaseTableRO};
use redb::Error;

pub struct UseCaseRepository<'a> {
    redb_table: Box<dyn UseCaseTable + 'a>,
}

impl<'a> UseCaseRepository<'a> {
    pub fn new(redb_table: Box<dyn UseCaseTable + 'a>) -> Self {
        UseCaseRepository { redb_table }
    }

    pub fn create(&mut self, use_case: &UseCase) -> Result<UseCase, Error> {
        self.redb_table.create(use_case)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error> {
        self.redb_table.get(id)
    }

    pub fn update(&mut self, use_case: &UseCase) -> Result<UseCase, Error> {
        self.redb_table.update(use_case)
    }

    pub fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.redb_table.delete(id)
    }

    pub fn get_relationships_of(
        &self,
        field: &UseCaseRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table.get_relationships_of(field, right_ids)
    }

    pub fn delete_all_relationships_with(
        &mut self,
        field: &UseCaseRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table.delete_all_relationships_with(field, right_ids)
    }

    pub fn set_relationships(
        &mut self,
        field: &UseCaseRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table.set_relationships(field, relationships)
    }
}

pub struct UseCaseRepositoryRO<'a> {
    redb_table: Box<dyn UseCaseTableRO + 'a>,
}

impl<'a> UseCaseRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn UseCaseTableRO + 'a>) -> Self {
        UseCaseRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error> {
        self.redb_table.get(id)
    }
}
