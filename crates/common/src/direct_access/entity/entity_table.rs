use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::Entity;
use crate::entities::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const ENTITY_TABLE: TableDefinition<EntityId, Bincode<Entity>> = TableDefinition::new("entity");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("field_from_entity_fields_junction");
const RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("relationship_from_entity_relationships_junction");
// backward relationships
const ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("entity_from_root_entities_junction");
const ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("entity_from_use_case_entities_junction");
const ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("entity_from_field_entity_junction");


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityRelationshipField  {
    Field,
    Relationship
}

pub trait EntityTable {
    fn create(&mut self, entity: &Entity) -> Result<Entity, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error>;
    fn update(&mut self, entity: &Entity) -> Result<Entity, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn get_relationships_of(&self, field: &EntityRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn delete_all_relationships_with(&mut self, field: &EntityRelationshipField, right_ids: &[EntityId]) -> Result<(), Error>;
    fn set_relationships(&mut self, field: &EntityRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<(), Error>;

}

pub trait EntityTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error>;
}

#[derive(Clone)]
pub struct EntityRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> EntityRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        EntityRedbTable {
            transaction,
        }
    }
}

impl<'a> EntityTable for EntityRedbTable<'a> {
    fn create(&mut self, entity: &Entity) -> Result<Entity, Error> {
        // retrieve the counter
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let counter = if let Some(counter) = counter_table.get(&"entity".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut table = self.transaction.open_table(ENTITY_TABLE)?;

        let entity = Entity {
            id: counter,
            ..entity.clone()
        };
        table.insert(entity.id, entity.clone())?;

        // update the counter
        counter_table.insert("entity".to_string(), counter)?;

        // add fields to junction table
        let mut junction_table = self.transaction.open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        junction_table.insert(entity.id, entity.fields.clone())?;

        // add relationships to junction table
        let mut junction_table = self.transaction.open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;
        junction_table.insert(entity.id, entity.relationships.clone())?;

        Ok(entity)
    }

    fn get(&self, id: &EntityId) -> Result<Option<Entity>, Error> {
        let table = self.transaction.open_table(ENTITY_TABLE)?;
        let guard = table.get(id)?;
        let entity = guard.map(|guard| guard.value().clone());

        if entity.is_none() {
            return Ok(None);
        }

        // get fields from junction table
        let junction_table = self.transaction.open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        let fields = junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default();

        // get relationships from junction table
        let junction_table = self.transaction.open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;
        let relationships = junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default();

        Ok(entity.map(|mut entity| {
            entity.fields = fields;
            entity.relationships = relationships;
            entity
        }))
    }

    fn update(&mut self, entity: &Entity) -> Result<Entity, Error> {
        // update the entity table
        let mut table = self.transaction.open_table(ENTITY_TABLE)?;
        table.insert(entity.id, entity)?;

        // update the junction table
        let mut junction_table = self.transaction.open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        junction_table.insert(entity.id, entity.fields.clone())?;

        // update the junction table
        let mut junction_table = self.transaction.open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;
        junction_table.insert(entity.id, entity.relationships.clone())?;

        Ok(entity.clone())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        // delete from entity table
        let mut table = self.transaction.open_table(ENTITY_TABLE)?;
        table.remove(id)?;

        // delete from junction table
        let mut junction_table = self.transaction.open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        junction_table.remove(id)?;

        // delete from junction table
        let mut junction_table = self.transaction.open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;
        junction_table.remove(id)?;

        // delete from backward junction tables, where the id may be in the Vec in the value
        let mut junction_table: redb::Table<'_, u64, Vec<u64>> = self.transaction.open_table(ENTITY_FROM_ROOT_ENTITIES_JUNCTION_TABLE)?;
        db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;

        let mut junction_table: redb::Table<'_, u64, Vec<u64>> = self.transaction.open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;

        let mut junction_table: redb::Table<'_, u64, Vec<u64>> = self.transaction.open_table(ENTITY_FROM_FIELD_ENTITY_JUNCTION_TABLE)?;
        db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;

        Ok(())
    }

    
    fn get_relationships_of(&self, field: &EntityRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition =
            match field {
                EntityRelationshipField::Field => FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE,
                EntityRelationshipField::Relationship => RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE,
            
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
    fn delete_all_relationships_with(&mut self, field: &EntityRelationshipField, right_ids: &[EntityId]) -> Result<(), Error> {
        // delete from junction table        
        let junction_table_definition =
            match field {
                EntityRelationshipField::Field => FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE,
                EntityRelationshipField::Relationship => RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE,
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


    fn set_relationships(&mut self, field: &EntityRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<(), Error> {
        let junction_table_definition =
            match field {
                EntityRelationshipField::Field => FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE,
                EntityRelationshipField::Relationship => RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE,
            
        };
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        for (left_id, entities) in relationships {
            junction_table.insert(left_id, entities)?;
        }
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
        let table = self.transaction.open_table(ENTITY_TABLE)?;
        let guard = table.get(id)?;
        let entity = guard.map(|guard| guard.value().clone());
        
        if entity.is_none() {
            return Ok(None);
        }

        // get fields from junction table
        let junction_table = self.transaction.open_table(FIELD_FROM_ENTITY_FIELDS_JUNCTION_TABLE)?;
        let fields = junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default();

        // get relationships from junction table
        let junction_table = self.transaction.open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;
        let relationships = junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default();

        Ok(entity.map(|mut entity| {
            entity.fields = fields;
            entity.relationships = relationships;
            entity
        }))
    }
}
