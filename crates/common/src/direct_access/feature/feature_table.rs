use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::Feature;
use crate::entities::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const FEATURE_TABLE: TableDefinition<EntityId, Bincode<Feature>> = TableDefinition::new("feature");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("use_case_from_feature_use_cases_junction");
// backward relationships
const FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("feature_from_root_features_junction");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureRelationshipField  {
    UseCases,
}

pub trait FeatureTable {
    fn create(&mut self, feature: &Feature) -> Result<Feature, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error>;
    fn update(&mut self, feature: &Feature) -> Result<Feature, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn get_relationships_of(&self, field: &FeatureRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn delete_all_relationships_with(&mut self, field: &FeatureRelationshipField, right_ids: &[EntityId]) -> Result<(), Error>;
    fn set_relationships(&mut self, field: &FeatureRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<(), Error>;
}

pub trait FeatureTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error>;
}

#[derive(Clone)]
pub struct FeatureRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> FeatureRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        FeatureRedbTable {
            transaction,
        }
    }
}

impl<'a> FeatureTable for FeatureRedbTable<'a> {
    fn create(&mut self, feature: &Feature) -> Result<Feature, Error> {
        // retrieve the counter
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let counter = if let Some(counter) = counter_table.get(&"feature".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut table = self.transaction.open_table(FEATURE_TABLE)?;

        let feature = Feature {
            id: counter,
            ..feature.clone()
        };
        table.insert(feature.id, feature.clone())?;

        // update the counter
        counter_table.insert("feature".to_string(), counter)?;

        // add use cases to junction table
        let mut junction_table = self.transaction.open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;
        junction_table.insert(feature.id, feature.use_cases.clone())?;

        Ok(feature)
    }

    fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error> {
        let table = self.transaction.open_table(FEATURE_TABLE)?;
        let guard = table.get(id)?;
        let feature = guard.map(|guard| guard.value().clone());

        if feature.is_none() {
            return Ok(None);
        }

        // get use cases from junction table
        let junction_table = self.transaction.open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;
        let use_cases = junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default();

        Ok(feature.map(|mut feature| {
            feature.use_cases = use_cases;
            feature
        }))
    }

    fn update(&mut self, feature: &Feature) -> Result<Feature, Error> {
        // update the feature table
        let mut table = self.transaction.open_table(FEATURE_TABLE)?;
        table.insert(feature.id, feature)?;

        // update the junction table
        let mut junction_table = self.transaction.open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;
        junction_table.insert(feature.id, feature.use_cases.clone())?;

        Ok(feature.clone())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        // delete from feature table
        let mut table = self.transaction.open_table(FEATURE_TABLE)?;
        table.remove(id)?;

        // delete from junction table
        let mut junction_table = self.transaction.open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;
        junction_table.remove(id)?;
        
        // delete from backward junction tables, where the id may be in the Vec in the value
        let mut junction_table: redb::Table<'_, u64, Vec<u64>> = self.transaction.open_table(FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE)?;
        db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;

        Ok(())
    }
    fn get_relationships_of(&self, field: &FeatureRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition =
            match field {
                FeatureRelationshipField::UseCases => USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE,
            
        };
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let mut relationship_iter = junction_table.iter()?;
        let mut relationships = vec![];
        while let Some(Ok((left_id, right_entities))) = relationship_iter.next() {
            let left_id = left_id.value();
            let right_entities = right_entities.value();
            if right_ids.iter().any(|entity_id| right_entities.contains(entity_id)) {
                relationships.push((left_id, right_entities));
            }
        }
        Ok(relationships)
    }

    /// Deletes all relationships between all root entities and the entities in `right_ids`.
    /// If the root has no relationship with an entity in `right_ids`, it is ignored.
    fn delete_all_relationships_with(&mut self, field: &FeatureRelationshipField, right_ids: &[EntityId]) -> Result<(), Error> {
        // delete from junction table        
        let junction_table_definition =
            match field {
                FeatureRelationshipField::UseCases => USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE,
        };
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        let mut relationship_iter = junction_table.iter()?;
        let mut junctions_to_modify: Vec<(EntityId, Vec<EntityId>)> = vec![];
        while let Some(Ok((left_id, right_entities))) = relationship_iter.next() {
            let left_id = left_id.value();
            let right_entities = right_entities.value();
            let entities_left: Vec<EntityId> = right_entities.clone().into_iter().filter(|entity_id| !right_ids.contains(entity_id)).collect();
            
            if entities_left.len() == right_entities.len() {
                continue;
            }
            junctions_to_modify.push((left_id, entities_left));

        }
        for (left_id, entities) in junctions_to_modify {
            junction_table.insert(left_id, entities)?;
        }      
        

        Ok(())
    }


    fn set_relationships(&mut self, field: &FeatureRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<(), Error> {
        let junction_table_definition =
            match field {
                FeatureRelationshipField::UseCases => USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE,
            
        };
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        for (left_id, entities) in relationships {
            junction_table.insert(left_id, entities)?;
        }
        Ok(())
    }
}

pub struct FeatureRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> FeatureRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        FeatureRedbTableRO { transaction }
    }
}

impl<'a> FeatureTableRO for FeatureRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error> {
        let table = self.transaction.open_table(FEATURE_TABLE)?;
        let guard = table.get(id)?;
        let feature = guard.map(|guard| guard.value().clone());

        if feature.is_none() {
            return Ok(None);
        }

        // get use cases from junction table
        let junction_table = self.transaction.open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;
        let use_cases = junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default();

        Ok(feature.map(|mut feature| {
            feature.use_cases = use_cases;
            feature
        }))
    }
}
