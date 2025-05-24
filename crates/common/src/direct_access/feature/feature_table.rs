use super::feature_repository::FeatureRelationshipField;
use super::feature_repository::FeatureTable;
use super::feature_repository::FeatureTableRO;
use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::Feature;
use crate::types::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const FEATURE_TABLE: TableDefinition<EntityId, Bincode<Feature>> = TableDefinition::new("feature");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("use_case_from_feature_use_cases_junction");
// backward relationships
const FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("feature_from_root_features_junction");

fn get_junction_table_definition(
    field: &FeatureRelationshipField,
) -> TableDefinition<EntityId, Vec<EntityId>> {
    match field {
        FeatureRelationshipField::UseCases => USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE,
    }
}

pub struct FeatureRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> FeatureRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        FeatureRedbTable { transaction }
    }
}

impl<'a> FeatureTable for FeatureRedbTable<'a> {
    fn create(&mut self, feature: &Feature) -> Result<Feature, Error> {
        let features = self.create_multi(&[feature.clone()])?;
        Ok(features.into_iter().next().unwrap())
    }

    fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error> {
        let features = self.get_multi(&[id.clone()])?;
        Ok(features.into_iter().next().unwrap())
    }

    fn update(&mut self, feature: &Feature) -> Result<Feature, Error> {
        let features = self.update_multi(&[feature.clone()])?;
        Ok(features.into_iter().next().unwrap())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.delete_multi(&[id.clone()])
    }

    fn create_multi(&mut self, features: &[Feature]) -> Result<Vec<Feature>, Error> {
        let mut created_features = Vec::new();
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let mut counter = if let Some(counter) = counter_table.get(&"feature".to_string())? {
            counter.value()
        } else {
            1
        };

        let mut feature_table = self.transaction.open_table(FEATURE_TABLE)?;
        let mut use_case_junction_table = self
            .transaction
            .open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;

        for feature in features {
            // if the id is default, create a new id
            let new_feature = if feature.id == EntityId::default() {
                Feature {
                    id: counter,
                    ..feature.clone()
                }
            } else {
                // ensure that the id is not already in use
                if feature_table.get(&feature.id)?.is_some() {
                    panic!(
                        "Feature id already in use while creating it: {:?}",
                        feature.id
                    );
                }
                feature.clone()
            };

            feature_table.insert(new_feature.id, new_feature.clone())?;
            use_case_junction_table.insert(new_feature.id, new_feature.use_cases.clone())?;
            created_features.push(new_feature);

            if feature.id == EntityId::default() {
                counter += 1;
            }
        }

        counter_table.insert("feature".to_string(), counter)?;

        Ok(created_features)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Feature>>, Error> {
        let mut features = Vec::new();
        let feature_table = self.transaction.open_table(FEATURE_TABLE)?;
        let use_case_junction_table = self
            .transaction
            .open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut feature_iter = feature_table.iter()?;
            let mut count = 0;

            while let Some(Ok((id, feature_data))) = feature_iter.next() {
                if count >= 1000 {
                    break;
                }

                let id = id.value();
                let mut feature = feature_data.value().clone();

                // get use cases from junction table
                let use_cases = use_case_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                feature.use_cases = use_cases;
                features.push(Some(feature));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let feature = if let Some(guard) = feature_table.get(id)? {
                    let mut feature = guard.value().clone();

                    // get use cases from junction table
                    let use_cases = use_case_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default();

                    feature.use_cases = use_cases;
                    Some(feature)
                } else {
                    None
                };
                features.push(feature);
            }
        }

        Ok(features)
    }

    fn update_multi(&mut self, features: &[Feature]) -> Result<Vec<Feature>, Error> {
        let mut updated_features = Vec::new();
        let mut feature_table = self.transaction.open_table(FEATURE_TABLE)?;
        let mut use_case_junction_table = self
            .transaction
            .open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;

        for feature in features {
            feature_table.insert(feature.id, feature)?;
            use_case_junction_table.insert(feature.id, feature.use_cases.clone())?;
            updated_features.push(feature.clone());
        }

        Ok(updated_features)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut feature_table = self.transaction.open_table(FEATURE_TABLE)?;
        let mut use_case_junction_table = self
            .transaction
            .open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;
        let mut feature_junction_table = self
            .transaction
            .open_table(FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE)?;

        for id in ids {
            feature_table.remove(id)?;
            use_case_junction_table.remove(id)?;
            db_helpers::delete_from_backward_junction_table(&mut feature_junction_table, id)?;
        }

        Ok(())
    }

    fn get_relationship(
        &self,
        id: &EntityId,
        field: &FeatureRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let guard = junction_table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()).unwrap_or_default())
    }

    fn get_relationships_from_right_ids(
        &self,
        field: &FeatureRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let mut relationship_iter = junction_table.iter()?;
        let mut relationships = vec![];
        while let Some(Ok((left_id, right_entities))) = relationship_iter.next() {
            let left_id = left_id.value();
            let right_entities = right_entities.value();
            if right_ids
                .iter()
                .any(|entity_id| right_entities.contains(entity_id))
            {
                relationships.push((left_id, right_entities));
            }
        }
        Ok(relationships)
    }

    fn set_relationship_multi(
        &mut self,
        field: &FeatureRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        for (left_id, entities) in relationships {
            junction_table.insert(left_id, entities)?;
        }
        Ok(())
    }

    fn set_relationship(
        &mut self,
        id: &EntityId,
        field: &FeatureRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        junction_table.insert(id.clone(), right_ids.to_vec())?;
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
        let features = self.get_multi(&[id.clone()])?;
        Ok(features.into_iter().next().unwrap())
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Feature>>, Error> {
        let mut features = Vec::new();
        let feature_table = self.transaction.open_table(FEATURE_TABLE)?;
        let use_case_junction_table = self
            .transaction
            .open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut feature_iter = feature_table.iter()?;
            let mut count = 0;

            while let Some(Ok((id, feature_data))) = feature_iter.next() {
                if count >= 1000 {
                    break;
                }

                let id = id.value();
                let mut feature = feature_data.value().clone();

                // get use cases from junction table
                let use_cases = use_case_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                feature.use_cases = use_cases;
                features.push(Some(feature));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let feature = if let Some(guard) = feature_table.get(id)? {
                    let mut feature = guard.value().clone();

                    // get use cases from junction table
                    let use_cases = use_case_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default();

                    feature.use_cases = use_cases;
                    Some(feature)
                } else {
                    None
                };
                features.push(feature);
            }
        }

        Ok(features)
    }
    fn get_relationship(
        &self,
        id: &EntityId,
        field: &FeatureRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let guard = junction_table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()).unwrap_or_default())
    }

    fn get_relationships_from_right_ids(
        &self,
        field: &FeatureRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let mut relationship_iter = junction_table.iter()?;
        let mut relationships = vec![];
        while let Some(Ok((left_id, right_entities))) = relationship_iter.next() {
            let left_id = left_id.value();
            let right_entities = right_entities.value();
            if right_ids
                .iter()
                .any(|entity_id| right_entities.contains(entity_id))
            {
                relationships.push((left_id, right_entities));
            }
        }
        Ok(relationships)
    }
}
