use crate::database::Bincode;
use crate::entities::EntityId;
use crate::entities::Root;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

use super::root_repository::{RootRelationshipField, RootTable, RootTableRO};

const ROOT_TABLE: TableDefinition<EntityId, Bincode<Root>> = TableDefinition::new("root");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("global_from_root_global_junction");
const ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("entity_from_root_entities_junction");
const FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("feature_from_root_features_junction");

pub struct RootRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> RootRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        RootRedbTable { transaction }
    }
}

impl<'a> RootTable for RootRedbTable<'a> {
    fn create(&mut self, root: &Root) -> Result<Root, Error> {
        let roots = self.create_multi(&[root.clone()])?;
        Ok(roots.into_iter().next().unwrap())
    }

    fn get(&self, id: &EntityId) -> Result<Option<Root>, Error> {
        let roots = self.get_multi(&[id.clone()])?;
        Ok(roots.into_iter().next().unwrap())
    }

    fn update(&mut self, root: &Root) -> Result<Root, Error> {
        let roots = self.update_multi(&[root.clone()])?;
        Ok(roots.into_iter().next().unwrap())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.delete_multi(&[id.clone()])
    }

    fn get_relationships_of(
        &self,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition = match field {
            RootRelationshipField::Global => GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE,
            RootRelationshipField::Entities => ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE,
            RootRelationshipField::Features => FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE,
        };
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

    /// Deletes all relationships between all root entities and the entities in `right_ids`.
    /// If the root has no relationship with an entity in `right_ids`, it is ignored.
    fn delete_all_relationships_with(
        &mut self,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        // delete from junction table
        let junction_table_definition = match field {
            RootRelationshipField::Global => GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE,
            RootRelationshipField::Entities => ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE,
            RootRelationshipField::Features => FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE,
        };
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        let mut relationship_iter = junction_table.iter()?;
        let mut junctions_to_modify: Vec<(EntityId, Vec<EntityId>)> = vec![];
        while let Some(Ok((left_id, right_entities))) = relationship_iter.next() {
            let left_id = left_id.value();
            let right_entities = right_entities.value();
            let entities_left: Vec<EntityId> = right_entities
                .clone()
                .into_iter()
                .filter(|entity_id| !right_ids.contains(entity_id))
                .collect();

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

    fn set_relationships(
        &mut self,
        field: &RootRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        let junction_table_definition = match field {
            RootRelationshipField::Global => GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE,
            RootRelationshipField::Entities => ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE,
            RootRelationshipField::Features => FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE,
        };
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        for (left_id, entities) in relationships {
            junction_table.insert(left_id, entities)?;
        }
        Ok(())
    }
    fn create_multi(&mut self, roots: &[Root]) -> Result<Vec<Root>, Error> {
        let mut created_roots = Vec::new();
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let mut counter = if let Some(counter) = counter_table.get(&"root".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut root_table = self.transaction.open_table(ROOT_TABLE)?;
        let mut global_junction_table = self
            .transaction
            .open_table(GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE)?;
        let mut entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE)?;
        let mut feature_junction_table = self
            .transaction
            .open_table(FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE)?;

        for root in roots {
            let new_root = Root {
                id: counter,
                ..root.clone()
            };
            root_table.insert(new_root.id, new_root.clone())?;
            global_junction_table.insert(new_root.id, vec![new_root.global] as Vec<EntityId>)?;
            entity_junction_table.insert(new_root.id, new_root.entities.clone())?;
            feature_junction_table.insert(new_root.id, new_root.features.clone())?;
            created_roots.push(new_root);
            counter += 1;
        }

        counter_table.insert("root".to_string(), counter)?;

        Ok(created_roots)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Root>>, Error> {
        let mut roots = Vec::new();
        let root_table = self.transaction.open_table(ROOT_TABLE)?;
        let global_junction_table = self
            .transaction
            .open_table(GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE)?;
        let entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE)?;
        let feature_junction_table = self
            .transaction
            .open_table(FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE)?;

        for id in ids {
            let root = if let Some(guard) = root_table.get(id)? {
                let mut root = guard.value().clone();

                // get globals from junction table
                let global: EntityId = global_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop()
                    .expect("root has no global");

                // get entities from junction table
                let entities = entity_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                // get features from junction table
                let features = feature_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                root.global = global;
                root.entities = entities;
                root.features = features;
                Some(root)
            } else {
                None
            };
            roots.push(root);
        }
        Ok(roots)
    }

    fn update_multi(&mut self, roots: &[Root]) -> Result<Vec<Root>, Error> {
        let mut updated_roots = Vec::new();
        let mut root_table = self.transaction.open_table(ROOT_TABLE)?;
        let mut global_junction_table = self
            .transaction
            .open_table(GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE)?;
        let mut entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE)?;
        let mut feature_junction_table = self
            .transaction
            .open_table(FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE)?;

        for root in roots {
            root_table.insert(root.id, root)?;
            global_junction_table.insert(root.id, vec![root.global] as Vec<EntityId>)?;
            entity_junction_table.insert(root.id, root.entities.clone())?;
            feature_junction_table.insert(root.id, root.features.clone())?;
            updated_roots.push(root.clone());
        }

        Ok(updated_roots)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut root_table = self.transaction.open_table(ROOT_TABLE)?;
        let mut global_junction_table = self
            .transaction
            .open_table(GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE)?;
        let mut entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE)?;
        let mut feature_junction_table = self
            .transaction
            .open_table(FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE)?;

        for id in ids {
            root_table.remove(id)?;
            global_junction_table.remove(id)?;
            entity_junction_table.remove(id)?;
            feature_junction_table.remove(id)?;
        }

        Ok(())
    }
}

pub struct RootRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> RootRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        RootRedbTableRO { transaction }
    }
}

impl<'a> RootTableRO for RootRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<Root>, Error> {
        let roots = self.get_multi(&[id.clone()])?;
        Ok(roots.into_iter().next().unwrap())
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Root>>, Error> {
        let mut roots = Vec::new();
        let root_table = self.transaction.open_table(ROOT_TABLE)?;
        let global_junction_table = self
            .transaction
            .open_table(GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE)?;
        let entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE)?;
        let feature_junction_table = self
            .transaction
            .open_table(FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE)?;

        for id in ids {
            let root = if let Some(guard) = root_table.get(id)? {
                let mut root = guard.value().clone();

                // get globals from junction table
                let global: EntityId = global_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop()
                    .expect("root has no global");

                // get entities from junction table
                let entities = entity_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                // get features from junction table
                let features = feature_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                root.global = global;
                root.entities = entities;
                root.features = features;
                Some(root)
            } else {
                None
            };
            roots.push(root);
        }
        Ok(roots)
    }
}
