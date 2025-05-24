use super::field_repository::FieldRelationshipField;
use super::field_repository::FieldTable;
use super::field_repository::FieldTableRO;
use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::Field;
use crate::types::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const FIELD_TABLE: TableDefinition<EntityId, Bincode<Field>> = TableDefinition::new("field");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("entity_from_field_entity_junction");
// backward relationships
const FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("field_from_entity_fields_junction");

fn get_junction_table_definition(
    field: &FieldRelationshipField,
) -> TableDefinition<EntityId, Vec<EntityId>> {
    match field {
        FieldRelationshipField::Entity => ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE,
    }
}

pub struct FieldRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> FieldRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        FieldRedbTable { transaction }
    }
}

impl<'a> FieldTable for FieldRedbTable<'a> {
    fn create(&mut self, field: &Field) -> Result<Field, Error> {
        let fields = self.create_multi(&[field.clone()])?;
        Ok(fields.into_iter().next().unwrap())
    }

    fn get(&self, id: &EntityId) -> Result<Option<Field>, Error> {
        let fields = self.get_multi(&[id.clone()])?;
        Ok(fields.into_iter().next().unwrap())
    }

    fn update(&mut self, field: &Field) -> Result<Field, Error> {
        let fields = self.update_multi(&[field.clone()])?;
        Ok(fields.into_iter().next().unwrap())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.delete_multi(&[id.clone()])
    }

    fn create_multi(&mut self, fields: &[Field]) -> Result<Vec<Field>, Error> {
        let mut created_fields = Vec::new();
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let mut counter = if let Some(counter) = counter_table.get(&"field".to_string())? {
            counter.value()
        } else {
            1
        };

        let mut field_table = self.transaction.open_table(FIELD_TABLE)?;
        let mut entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;

        for field in fields {
            // if the id is default, create a new id

            let new_field = if field.id == EntityId::default() {
                Field {
                    id: counter,
                    ..field.clone()
                }
            } else {
                // ensure that the id is not already in use
                if field_table.get(&field.id)?.is_some() {
                    panic!("Field id already in use while creating it: {:?}", field.id);
                }
                field.clone()
            };
            field_table.insert(new_field.id, new_field.clone())?;
            entity_junction_table.insert(
                new_field.id,
                new_field
                    .entity
                    .clone()
                    .into_iter()
                    .collect::<Vec<EntityId>>(),
            )?;
            created_fields.push(new_field);

            if field.id == EntityId::default() {
                counter += 1;
            }
        }

        counter_table.insert("field".to_string(), counter)?;

        Ok(created_fields)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Field>>, Error> {
        let mut fields = Vec::new();
        let field_table = self.transaction.open_table(FIELD_TABLE)?;
        let entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut field_iter = field_table.iter()?;
            let mut count = 0;

            while let Some(Ok((id, field_data))) = field_iter.next() {
                if count >= 1000 {
                    break;
                }

                let id = id.value();
                let mut field = field_data.value().clone();

                // get entity from junction table
                let entity: Option<EntityId> = entity_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop();

                field.entity = entity;
                fields.push(Some(field));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let field = if let Some(guard) = field_table.get(id)? {
                    let mut field = guard.value().clone();

                    // get entity from junction table
                    let entity: Option<EntityId> = entity_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default()
                        .pop();

                    field.entity = entity;
                    Some(field)
                } else {
                    None
                };
                fields.push(field);
            }
        }

        Ok(fields)
    }

    fn update_multi(&mut self, fields: &[Field]) -> Result<Vec<Field>, Error> {
        let mut updated_fields = Vec::new();
        let mut field_table = self.transaction.open_table(FIELD_TABLE)?;
        let mut entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;

        for field in fields {
            field_table.insert(field.id, field)?;
            entity_junction_table.insert(
                field.id,
                field.entity.clone().into_iter().collect::<Vec<EntityId>>(),
            )?;
            updated_fields.push(field.clone());
        }

        Ok(updated_fields)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut field_table = self.transaction.open_table(FIELD_TABLE)?;
        let mut entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;
        let mut field_junction_table = self
            .transaction
            .open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;

        for id in ids {
            field_table.remove(id)?;
            entity_junction_table.remove(id)?;
            db_helpers::delete_from_backward_junction_table(&mut field_junction_table, id)?;
        }

        Ok(())
    }
    fn get_relationship(
        &self,
        id: &EntityId,
        field: &FieldRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let guard = junction_table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()).unwrap_or_default())
    }

    fn get_relationships_from_right_ids(
        &self,
        field: &FieldRelationshipField,
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
        field: &FieldRelationshipField,
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
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        junction_table.insert(id.clone(), right_ids.to_vec())?;
        Ok(())
    }
}

pub struct FieldRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> FieldRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        FieldRedbTableRO { transaction }
    }
}

impl<'a> FieldTableRO for FieldRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<Field>, Error> {
        let fields = self.get_multi(&[id.clone()])?;
        Ok(fields.into_iter().next().unwrap())
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Field>>, Error> {
        let mut fields = Vec::new();
        let field_table = self.transaction.open_table(FIELD_TABLE)?;
        let entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut field_iter = field_table.iter()?;
            let mut count = 0;

            while let Some(Ok((id, field_data))) = field_iter.next() {
                if count >= 1000 {
                    break;
                }

                let id = id.value();
                let mut field = field_data.value().clone();

                // get entity from junction table
                let entity: Option<EntityId> = entity_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop();

                field.entity = entity;
                fields.push(Some(field));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let field = if let Some(guard) = field_table.get(id)? {
                    let mut field = guard.value().clone();

                    // get entity from junction table
                    let entity: Option<EntityId> = entity_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default()
                        .pop();

                    field.entity = entity;
                    Some(field)
                } else {
                    None
                };
                fields.push(field);
            }
        }

        Ok(fields)
    }

    fn get_relationship(
        &self,
        id: &EntityId,
        field: &FieldRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let guard = junction_table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()).unwrap_or_default())
    }

    fn get_relationships_from_right_ids(
        &self,
        field: &FieldRelationshipField,
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
