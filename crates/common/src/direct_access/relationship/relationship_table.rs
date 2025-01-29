use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::Relationship;
use crate::entities::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const RELATIONSHIP_TABLE: TableDefinition<EntityId, Bincode<Relationship>> = TableDefinition::new("relationship");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// backward relationships
const RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("relationship_from_entity_relationships_junction");

pub trait RelationshipTable {
    fn create(&mut self, relationship: &Relationship) -> Result<Relationship, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error>;
    fn update(&mut self, relationship: &Relationship) -> Result<Relationship, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
}

pub trait RelationshipTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error>;
}

#[derive(Clone)]
pub struct RelationshipRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> RelationshipRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        RelationshipRedbTable { transaction }
    }
}

impl<'a> RelationshipTable for RelationshipRedbTable<'a> {
    fn create(&mut self, relationship: &Relationship) -> Result<Relationship, Error> {
        // retrieve the counter
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let counter = if let Some(counter) = counter_table.get(&"relationship".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut table = self.transaction.open_table(RELATIONSHIP_TABLE)?;

        let new_relationship = Relationship {
            id: counter,
            ..relationship.clone()
        };
        table.insert(new_relationship.id, new_relationship.clone())?;

        // update the counter
        counter_table.insert("relationship".to_string(), counter)?;

        Ok(new_relationship)
    }

    fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error> {
        let table = self.transaction.open_table(RELATIONSHIP_TABLE)?;
        let guard = table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()))
    }

    fn update(&mut self, relationship: &Relationship) -> Result<Relationship, Error> {
        let mut table = self.transaction.open_table(RELATIONSHIP_TABLE)?;
        table.insert(relationship.id, relationship)?;
        Ok(relationship.clone())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        let mut table = self.transaction.open_table(RELATIONSHIP_TABLE)?;
        table.remove(id)?;

        // delete from backward junction tables, where the id may be in the Vec in the value
        let mut junction_table: redb::Table<'_, u64, Vec<u64>> = self.transaction.open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;
        db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;

        Ok(())
    }
}

pub struct RelationshipRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> RelationshipRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        RelationshipRedbTableRO { transaction }
    }
}

impl<'a> RelationshipTableRO for RelationshipRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error> {
        let table = self.transaction.open_table(RELATIONSHIP_TABLE)?;
        let guard = table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()))
    }
}
