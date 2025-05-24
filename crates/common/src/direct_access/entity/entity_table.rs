use super::entity_repository::EntityRelationshipField;
use super::entity_repository::EntityTable;
use super::entity_repository::EntityTableRO;
use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::Entity;
use crate::types::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const ENTITY_TABLE: TableDefinition<EntityId, Bincode<Entity>> = TableDefinition::new("entity");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("field_from_entity_fields_junction");
const RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE: TableDefinition<
    EntityId,
    Vec<EntityId>,
> = TableDefinition::new("relationship_from_entity_relationships_junction");
// backward relationships
const ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("entity_from_root_entities_junction");
const ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("entity_from_use_case_entities_junction");
const ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("entity_from_field_entity_junction");
const ENTITY_FROM_ENTITY_PARENT_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("entity_from_entity_parent_junction");

fn get_junction_table_definition(
    field: &EntityRelationshipField,
) -> TableDefinition<EntityId, Vec<EntityId>> {
    match field {
        EntityRelationshipField::Parent => ENTITY_FROM_ENTITY_PARENT_JUNCTION_TABLE,
        EntityRelationshipField::Fields => FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE,
        EntityRelationshipField::Relationships => {
            RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE
        }
    }
}

pub struct EntityRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> EntityRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        EntityRedbTable { transaction }
    }
}

impl<'a> EntityTable for EntityRedbTable<'a> {
    fn create(&mut self, entity: &Entity) -> Result<Entity, Error> {
        let entities = self.create_multi(&[entity.clone()])?;
        Ok(entities.into_iter().next().unwrap())
    }

    fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error> {
        let entities = self.get_multi(&[id.clone()])?;
        Ok(entities.into_iter().next().unwrap())
    }

    fn update(&mut self, entity: &Entity) -> Result<Entity, Error> {
        let entities = self.update_multi(&[entity.clone()])?;
        Ok(entities.into_iter().next().unwrap())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.delete_multi(&[id.clone()])
    }

    fn create_multi(&mut self, entities: &[Entity]) -> Result<Vec<Entity>, Error> {
        let mut created_entities = Vec::new();
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let mut counter = if let Some(counter) = counter_table.get(&"entity".to_string())? {
            counter.value()
        } else {
            1
        };

        let mut entity_table = self.transaction.open_table(ENTITY_TABLE)?;
        // open junction tables
        let mut parent_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ENTITY_PARENT_JUNCTION_TABLE)?;
        let mut field_junction_table = self
            .transaction
            .open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        let mut relationship_junction_table = self
            .transaction
            .open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;

        for entity in entities {
            // if the id is default, create a new id
            let new_entity = if entity.id == EntityId::default() {
                Entity {
                    id: counter,
                    ..entity.clone()
                }
            } else {
                // ensure that the id is not already in use
                if entity_table.get(&entity.id)?.is_some() {
                    panic!(
                        "Entity id already in use while creating it: {:?}",
                        entity.id
                    );
                }
                entity.clone()
            };
            entity_table.insert(new_entity.id, new_entity.clone())?;
            // insert into junction tables
            if let Some(parent) = new_entity.parent {
                parent_junction_table.insert(new_entity.id, vec![parent])?;
            }
            field_junction_table.insert(new_entity.id, new_entity.fields.clone())?;
            relationship_junction_table.insert(new_entity.id, new_entity.relationships.clone())?;
            created_entities.push(new_entity);

            if entity.id == EntityId::default() {
                counter += 1;
            }
        }

        counter_table.insert("entity".to_string(), counter)?;

        Ok(created_entities)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Entity>>, Error> {
        let mut entities = Vec::new();
        let entity_table = self.transaction.open_table(ENTITY_TABLE)?;
        // open junction tables
        let parent_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ENTITY_PARENT_JUNCTION_TABLE)?;
        let field_junction_table = self
            .transaction
            .open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        let relationship_junction_table = self
            .transaction
            .open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut entity_iter = entity_table.iter()?;
            let mut count = 0;

            while let Some(Ok((id, entity_data))) = entity_iter.next() {
                if count >= 1000 {
                    break;
                }

                let id = id.value();
                let mut entity = entity_data.value().clone();

                // get parent from junction table
                let parent = parent_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop();

                // get fields from junction table
                let fields = field_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                // get relationships from junction table
                let relationships = relationship_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                entity.parent = parent;
                entity.fields = fields;
                entity.relationships = relationships;
                entities.push(Some(entity));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let entity = if let Some(guard) = entity_table.get(id)? {
                    let mut entity = guard.value().clone();

                    // get parent from junction table
                    let parent = parent_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default()
                        .pop();

                    // get fields from junction table
                    let fields = field_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default();

                    // get relationships from junction table
                    let relationships = relationship_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default();

                    entity.parent = parent;
                    entity.fields = fields;
                    entity.relationships = relationships;
                    Some(entity)
                } else {
                    None
                };
                entities.push(entity);
            }
        }

        Ok(entities)
    }

    fn update_multi(&mut self, entities: &[Entity]) -> Result<Vec<Entity>, Error> {
        let mut updated_entities = Vec::new();
        let mut entity_table = self.transaction.open_table(ENTITY_TABLE)?;
        // open junction tables
        let mut parent_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ENTITY_PARENT_JUNCTION_TABLE)?;
        let mut field_junction_table = self
            .transaction
            .open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        let mut relationship_junction_table = self
            .transaction
            .open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;

        for entity in entities {
            entity_table.insert(entity.id, entity)?;
            // update parent in junction table
            if let Some(parent) = entity.parent {
                parent_junction_table.insert(entity.id, vec![parent])?;
            } else {
                parent_junction_table.remove(entity.id)?;
            }
            field_junction_table.insert(entity.id, entity.fields.clone())?;
            relationship_junction_table.insert(entity.id, entity.relationships.clone())?;
            updated_entities.push(entity.clone());
        }

        Ok(updated_entities)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut entity_table = self.transaction.open_table(ENTITY_TABLE)?;
        // open junction tables
        let mut parent_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ENTITY_PARENT_JUNCTION_TABLE)?;
        let mut field_junction_table = self
            .transaction
            .open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        let mut relationship_junction_table = self
            .transaction
            .open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;
        // open backward junction tables
        let mut root_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE)?;
        let mut use_case_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        let mut field_entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;

        for id in ids {
            entity_table.remove(id)?;
            // remove from junction tables
            parent_junction_table.remove(id)?;
            field_junction_table.remove(id)?;
            relationship_junction_table.remove(id)?;
            db_helpers::delete_from_backward_junction_table(&mut parent_junction_table, id)?;
            db_helpers::delete_from_backward_junction_table(&mut root_junction_table, id)?;
            db_helpers::delete_from_backward_junction_table(&mut use_case_junction_table, id)?;
            db_helpers::delete_from_backward_junction_table(&mut field_entity_junction_table, id)?;
        }

        Ok(())
    }

    fn get_relationship(
        &self,
        id: &EntityId,
        field: &EntityRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let guard = junction_table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()).unwrap_or_default())
    }

    fn get_relationships_from_right_ids(
        &self,
        field: &EntityRelationshipField,
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
        field: &EntityRelationshipField,
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
        field: &EntityRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        junction_table.insert(id.clone(), right_ids.to_vec())?;
        Ok(())
    }
}

pub struct EntityRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> EntityRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        EntityRedbTableRO { transaction }
    }
}

impl<'a> EntityTableRO for EntityRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error> {
        let entities = self.get_multi(&[id.clone()])?;
        Ok(entities.into_iter().next().unwrap())
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Entity>>, Error> {
        let mut entities = Vec::new();
        let entity_table = self.transaction.open_table(ENTITY_TABLE)?;
        // open junction tables
        let parent_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_ENTITY_PARENT_JUNCTION_TABLE)?;
        let field_junction_table = self
            .transaction
            .open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        let relationship_junction_table = self
            .transaction
            .open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut entity_iter = entity_table.iter()?;
            let mut count = 0;

            while let Some(Ok((id, entity_data))) = entity_iter.next() {
                if count >= 1000 {
                    break;
                }

                let id = id.value();
                let mut entity = entity_data.value().clone();

                // get parent from junction table
                let parent = parent_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop();

                // get fields from junction table
                let fields = field_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                // get relationships from junction table
                let relationships = relationship_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                entity.parent = parent;
                entity.fields = fields;
                entity.relationships = relationships;
                entities.push(Some(entity));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let entity = if let Some(guard) = entity_table.get(id)? {
                    let mut entity = guard.value().clone();

                    // get parent from junction table
                    let parent = parent_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default()
                        .pop();

                    // get fields from junction table
                    let fields = field_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default();

                    // get relationships from junction table
                    let relationships = relationship_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default();

                    entity.parent = parent;
                    entity.fields = fields;
                    entity.relationships = relationships;
                    Some(entity)
                } else {
                    None
                };
                entities.push(entity);
            }
        }

        Ok(entities)
    }

    fn get_relationship(
        &self,
        id: &EntityId,
        field: &EntityRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let guard = junction_table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()).unwrap_or_default())
    }

    fn get_relationships_from_right_ids(
        &self,
        field: &EntityRelationshipField,
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
