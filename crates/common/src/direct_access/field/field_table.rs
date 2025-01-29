use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::Field;
use crate::entities::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const FIELD_TABLE: TableDefinition<EntityId, Bincode<Field>> = TableDefinition::new("field");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("entity_from_field_entity_junction");
// backward relationships
const FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("field_from_entity_fields_junction");

pub enum FieldRelationshipField {
    Entity
}

pub trait FieldTable {
    fn create(&mut self, field: &Field) -> Result<Field, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Field>, Error>;
    fn update(&mut self, field: &Field) -> Result<Field, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn get_relationships_of(&self, field: &FieldRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn delete_all_relationships_with(&mut self, field: &FieldRelationshipField, right_ids: &[EntityId]) -> Result<(), Error>;
    fn set_relationships(&mut self, field: &FieldRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<(), Error>;

}

pub trait FieldTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Field>, Error>;
}

#[derive(Clone)]
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
        // retrieve the counter
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let counter = if let Some(counter) = counter_table.get(&"field".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut table = self.transaction.open_table(FIELD_TABLE)?;

        let new_field = Field {
            id: counter,
            ..field.clone()
        };
        table.insert(new_field.id, new_field.clone())?;

        // update the counter
        counter_table.insert("field".to_string(), counter)?;

        // add entity to junction table
        let mut junction_table = self.transaction.open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;
        junction_table.insert(new_field.id, new_field.entity.clone().into_iter().collect::<Vec<EntityId>>())?;

        Ok(new_field)
    }

    fn get(&self, id: &EntityId) -> Result<Option<Field>, Error> {
        let table = self.transaction.open_table(FIELD_TABLE)?;
        let guard = table.get(id)?;
        let field = guard.map(|guard| guard.value().clone());

        if field.is_none() {
            return Ok(None);
        }

        // get entities from junction table
        let junction_table = self.transaction.open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;
        let entity: Option<EntityId> = junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default().pop();

        Ok(field.map(|mut field| {
            field.entity = entity;
            field
        }))
    }

    fn update(&mut self, field: &Field) -> Result<Field, Error> {
        // update the field table
        let mut table = self.transaction.open_table(FIELD_TABLE)?;
        table.insert(field.id, field)?;

        // update the junction table
        let mut junction_table = self.transaction.open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;
        junction_table.insert(field.id, field.entity.clone().into_iter().collect::<Vec<EntityId>>())?;

        Ok(field.clone())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        // delete from field table
        let mut table = self.transaction.open_table(FIELD_TABLE)?;
        table.remove(id)?;

        // delete from junction table
        let mut junction_table = self.transaction.open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;
        junction_table.remove(id)?;

        // delete from backward junction tables, where the id may be in the Vec in the value
        let mut junction_table: redb::Table<'_, u64, Vec<u64>> = self.transaction.open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;

        Ok(())
    }
    
    fn get_relationships_of(&self, field: &FieldRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition =
            match field {
                FieldRelationshipField::Entity => ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE,
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
    
    fn delete_all_relationships_with(&mut self, field: &FieldRelationshipField, right_ids: &[EntityId]) -> Result<(), Error> {
        // delete from junction table        
        let junction_table_definition =
            match field {
                FieldRelationshipField::Entity => ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE,
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
    
    fn set_relationships(&mut self, field: &FieldRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<(), Error> {
        let junction_table_definition =
            match field {
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
        let table = self.transaction.open_table(FIELD_TABLE)?;
        let guard = table.get(id)?;
        let field = guard.map(|guard| guard.value().clone());

        if field.is_none() {
            return Ok(None);
        }

        // get entities from junction table
        let junction_table = self.transaction.open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;
        let entity: Option<EntityId> = junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default().pop();

        Ok(field.map(|mut field| {
            field.entity = entity;
            field
        }))
    }
}
