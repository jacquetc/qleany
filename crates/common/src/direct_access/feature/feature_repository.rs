use crate::entities::{EntityId, Feature};

use super::feature_table::{FeatureRelationshipField, FeatureTable, FeatureTableRO};
use redb::Error;

pub struct FeatureRepository<'a> {
    redb_table: Box<dyn FeatureTable + 'a>,
}

impl<'a> FeatureRepository<'a> {
    pub fn new(redb_table: Box<dyn FeatureTable + 'a>) -> Self {
        FeatureRepository { redb_table }
    }

    pub fn create(&mut self, feature: &Feature) -> Result<Feature, Error> {
        self.redb_table.create(feature)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error> {
        self.redb_table.get(id)
    }

    pub fn update(&mut self, feature: &Feature) -> Result<Feature, Error> {
        self.redb_table.update(feature)
    }

    pub fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.redb_table.delete(id)
    }

    pub fn get_relationships_of(
        &self,
        field: &FeatureRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table.get_relationships_of(field, right_ids)
    }

    pub fn delete_all_relationships_with(
        &mut self,
        field: &FeatureRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table.delete_all_relationships_with(field, right_ids)
    }

    pub fn set_relationships(
        &mut self,
        field: &FeatureRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table.set_relationships(field, relationships)
    }

}

pub struct FeatureRepositoryRO<'a> {
    redb_table: Box<dyn FeatureTableRO + 'a>,
}

impl<'a> FeatureRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn FeatureTableRO + 'a>) -> Self {
        FeatureRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error> {
        self.redb_table.get(id)
    }
}
