use super::root_repository::{RootRelationshipField, RootTable, RootTableRO};
use crate::database::Bincode;
use crate::entities::Root;
use crate::types::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const ROOT_TABLE: TableDefinition<EntityId, Bincode<Root>> = TableDefinition::new("root");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("global_from_root_global_junction");
const ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("entity_from_root_entities_junction");
const FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("feature_from_root_features_junction");

const FILE_FROM_ROOT_FILES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("file_from_root_files_junction");

fn get_junction_table_definition(
    field: &RootRelationshipField,
) -> TableDefinition<EntityId, Vec<EntityId>> {
    match field {
        RootRelationshipField::Global => GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE,
        RootRelationshipField::Entities => ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE,
        RootRelationshipField::Features => FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE,
        RootRelationshipField::Files => FILE_FROM_ROOT_FILES_JUNCTION_TABLE,
    }
}

pub struct RootRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> RootRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        RootRedbTable { transaction }
    }

    pub fn init_tables(transaction: &WriteTransaction) -> Result<(), Error> {
        transaction.open_table(ROOT_TABLE)?;
        transaction.open_table(COUNTER_TABLE)?;
        transaction.open_table(GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE)?;
        transaction.open_table(ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE)?;
        transaction.open_table(FEATURE_FROM_ROOT_FEATURES_JUNCTION_TABLE)?;
        transaction.open_table(FILE_FROM_ROOT_FILES_JUNCTION_TABLE)?;
        Ok(())
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

    fn get_relationships_from_right_ids(
        &self,
        field: &RootRelationshipField,
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

    fn set_relationship(
        &mut self,
        id: &EntityId,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.set_relationship_multi(field, vec![(id.clone(), right_ids.to_vec())])
    }

    fn set_relationship_multi(
        &mut self,
        field: &RootRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        let junction_table_definition = get_junction_table_definition(field);
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
            counter.value()
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
            // if the id is default, create a new id
            let new_root = if root.id == EntityId::default() {
                Root {
                    id: counter,
                    ..root.clone()
                }
            } else {
                // ensure that the id is not already in use
                if root_table.get(&root.id)?.is_some() {
                    panic!("Root id already in use while creating it: {}", root.id);
                }
                root.clone()
            };
            // one-to-one constraint check: ensure global is not already referenced by another root
            {
                let mut iter = global_junction_table.iter()?;
                while let Some(Ok((existing_root_id, global_ids))) = iter.next() {
                    let existing_root_id = existing_root_id.value();
                    if existing_root_id != new_root.id
                        && global_ids.value().contains(&new_root.global)
                    {
                        panic!(
                            "One-to-one constraint violation: Global {} is already referenced by Root {}",
                            new_root.global, existing_root_id
                        );
                    }
                }
            }
            root_table.insert(new_root.id, new_root.clone())?;
            global_junction_table.insert(new_root.id, vec![new_root.global] as Vec<EntityId>)?;
            entity_junction_table.insert(new_root.id, new_root.entities.clone())?;
            feature_junction_table.insert(new_root.id, new_root.features.clone())?;
            created_roots.push(new_root);

            if root.id == EntityId::default() {
                counter += 1;
            }
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

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut root_iter = root_table.iter()?;
            let mut count = 0;

            while let Some(Ok((id, root_data))) = root_iter.next() {
                if count >= 1000 {
                    break;
                }

                let id = id.value();
                let mut root = root_data.value().clone();

                // get globals from junction table
                let global: EntityId = global_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop()
                    .expect("root has no global");

                // get entities from junction table
                let entities = entity_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                // get features from junction table
                let features = feature_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                root.global = global;
                root.entities = entities;
                root.features = features;
                roots.push(Some(root));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
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
            // one-to-one constraint check: ensure global is not already referenced by another root
            {
                let mut iter = global_junction_table.iter()?;
                while let Some(Ok((existing_root_id, global_ids))) = iter.next() {
                    let existing_root_id = existing_root_id.value();
                    if existing_root_id != root.id && global_ids.value().contains(&root.global) {
                        panic!(
                            "One-to-one constraint violation: Global {} is already referenced by Root {}",
                            root.global, existing_root_id
                        );
                    }
                }
            }
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

    fn get_relationship(
        &self,
        id: &EntityId,
        field: &RootRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let relationship: Vec<EntityId> = junction_table
            .get(id)?
            .map(|guard| guard.value().clone())
            .unwrap_or_default();
        Ok(relationship)
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

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut root_iter = root_table.iter()?;
            let mut count = 0;

            while let Some(Ok((id, root_data))) = root_iter.next() {
                if count >= 1000 {
                    break;
                }

                let id = id.value();
                let mut root = root_data.value().clone();

                // get globals from junction table
                let global: EntityId = global_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop()
                    .expect("root has no global");

                // get entities from junction table
                let entities = entity_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                // get features from junction table
                let features = feature_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                root.global = global;
                root.entities = entities;
                root.features = features;
                roots.push(Some(root));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
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
        }

        Ok(roots)
    }

    fn get_relationship(
        &self,
        id: &EntityId,
        field: &RootRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let relationship: Vec<EntityId> = junction_table
            .get(id)?
            .map(|guard| guard.value().clone())
            .unwrap_or_default();
        Ok(relationship)
    }

    fn get_relationships_from_right_ids(
        &self,
        field: &RootRelationshipField,
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
