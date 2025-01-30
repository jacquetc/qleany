use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::EntityId;
use crate::entities::Relationship;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

use super::relationship_repository;
use super::relationship_repository::RelationshipTable;
use super::relationship_repository::RelationshipTableRO;

const RELATIONSHIP_TABLE: TableDefinition<EntityId, Bincode<Relationship>> =
    TableDefinition::new("relationship");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// backward relationships
const RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE: TableDefinition<
    EntityId,
    Vec<EntityId>,
> = TableDefinition::new("relationship_from_entity_relationships_junction");

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
        let relationships = self.create_multi(&[relationship.clone()])?;
        Ok(relationships.into_iter().next().unwrap())
    }

    fn get(&self, id: &EntityId) -> Result<Option<Relationship>, Error> {
        let relationships = self.get_multi(&[id.clone()])?;
        Ok(relationships.into_iter().next().unwrap())
    }

    fn update(&mut self, relationship: &Relationship) -> Result<Relationship, Error> {
        let relationships = self.update_multi(&[relationship.clone()])?;
        Ok(relationships.into_iter().next().unwrap())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.delete_multi(&[id.clone()])
    }

    fn create_multi(&mut self, relationships: &[Relationship]) -> Result<Vec<Relationship>, Error> {
        let mut created_relationships = Vec::new();
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let mut counter = if let Some(counter) = counter_table.get(&"relationship".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut relationship_table = self.transaction.open_table(RELATIONSHIP_TABLE)?;

        for relationship in relationships {
            let new_relationship = Relationship {
                id: counter,
                ..relationship.clone()
            };
            relationship_table.insert(new_relationship.id, new_relationship.clone())?;
            created_relationships.push(new_relationship);
            counter += 1;
        }

        counter_table.insert("relationship".to_string(), counter)?;

        Ok(created_relationships)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Relationship>>, Error> {
        let mut relationships = Vec::new();
        let relationship_table = self.transaction.open_table(RELATIONSHIP_TABLE)?;

        for id in ids {
            let relationship = if let Some(guard) = relationship_table.get(id)? {
                Some(guard.value().clone())
            } else {
                None
            };
            relationships.push(relationship);
        }
        Ok(relationships)
    }

    fn update_multi(&mut self, relationships: &[Relationship]) -> Result<Vec<Relationship>, Error> {
        let mut updated_relationships = Vec::new();
        let mut relationship_table = self.transaction.open_table(RELATIONSHIP_TABLE)?;

        for relationship in relationships {
            relationship_table.insert(relationship.id, relationship)?;
            updated_relationships.push(relationship.clone());
        }

        Ok(updated_relationships)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut relationship_table = self.transaction.open_table(RELATIONSHIP_TABLE)?;
        let mut relationship_from_entity_relationships_junction_table = self
            .transaction
            .open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;

        for id in ids {
            relationship_table.remove(id)?;
            db_helpers::delete_from_backward_junction_table(
                &mut relationship_from_entity_relationships_junction_table,
                id,
            )?;
        }

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

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Relationship>>, Error> {
        let mut relationships = Vec::new();
        let relationship_table = self.transaction.open_table(RELATIONSHIP_TABLE)?;

        for id in ids {
            let relationship = if let Some(guard) = relationship_table.get(id)? {
                Some(guard.value().clone())
            } else {
                None
            };
            relationships.push(relationship);
        }
        Ok(relationships)
    }
}
