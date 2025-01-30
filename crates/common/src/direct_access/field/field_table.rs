use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::EntityId;
use crate::entities::Field;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

use super::field_repository::FieldRelationshipField;
use super::field_repository::FieldTable;
use super::field_repository::FieldTableRO;

const FIELD_TABLE: TableDefinition<EntityId, Bincode<Field>> = TableDefinition::new("field");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("entity_from_field_entity_junction");
// backward relationships
const FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("field_from_entity_fields_junction");

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
            counter.value() + 1
        } else {
            1
        };

        let mut field_table = self.transaction.open_table(FIELD_TABLE)?;
        let mut entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;

        for field in fields {
            let new_field = Field {
                id: counter,
                ..field.clone()
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
            counter += 1;
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

    fn get_relationships_of(
        &self,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition = match field {
            FieldRelationshipField::Entity => ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE,
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

    fn delete_all_relationships_with(
        &mut self,
        field: &FieldRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        // delete from junction table
        let junction_table_definition = match field {
            FieldRelationshipField::Entity => ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE,
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
        field: &FieldRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        let junction_table_definition = match field {
            FieldRelationshipField::Entity => ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE,
        };
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        for (left_id, entities) in relationships {
            junction_table.insert(left_id, entities)?;
        }
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
        Ok(fields)
    }
}
