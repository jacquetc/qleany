use super::relationship_repository::RelationshipTableRO;
use super::relationship_repository::{RelationshipRelationshipField, RelationshipTable};
use crate::database::Bincode;
use crate::database::db_helpers;
use crate::entities::Relationship;
use crate::types::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const RELATIONSHIP_TABLE: TableDefinition<EntityId, Bincode<Relationship>> =
    TableDefinition::new("relationship");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const ENTITY_FROM_RELATIONSHIP_LEFT_ENTITY_JUNCTION_TABLE: TableDefinition<
    EntityId,
    Vec<EntityId>,
> = TableDefinition::new("entity_from_relationship_left_entity_junction");
const ENTITY_FROM_RELATIONSHIP_RIGHT_ENTITY_JUNCTION_TABLE: TableDefinition<
    EntityId,
    Vec<EntityId>,
> = TableDefinition::new("entity_from_relationship_right_entity_junction");
// backward relationships
const RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE: TableDefinition<
    EntityId,
    Vec<EntityId>,
> = TableDefinition::new("relationship_from_entity_relationships_junction");

fn get_junction_table_definition(
    field: &RelationshipRelationshipField,
) -> TableDefinition<EntityId, Vec<EntityId>> {
    match field {
        RelationshipRelationshipField::LeftEntity => {
            ENTITY_FROM_RELATIONSHIP_LEFT_ENTITY_JUNCTION_TABLE
        }
        RelationshipRelationshipField::RightEntity => {
            ENTITY_FROM_RELATIONSHIP_RIGHT_ENTITY_JUNCTION_TABLE
        }
    }
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
            counter.value()
        } else {
            1
        };

        let mut relationship_table = self.transaction.open_table(RELATIONSHIP_TABLE)?;
        let mut left_entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_RELATIONSHIP_LEFT_ENTITY_JUNCTION_TABLE)?;
        let mut right_entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_RELATIONSHIP_RIGHT_ENTITY_JUNCTION_TABLE)?;

        for relationship in relationships {
            // if the id is default, create a new id
            let new_relationship = if relationship.id == EntityId::default() {
                Relationship {
                    id: counter,
                    ..relationship.clone()
                }
            } else {
                // ensure that the id is not already in use
                if relationship_table.get(&relationship.id)?.is_some() {
                    panic!(
                        "Relationship id already in use while creating it: {}",
                        relationship.id
                    );
                }
                relationship.clone()
            };
            relationship_table.insert(new_relationship.id, new_relationship.clone())?;
            left_entity_junction_table.insert(
                new_relationship.id,
                vec![new_relationship.left_entity] as Vec<EntityId>,
            )?;
            right_entity_junction_table.insert(
                new_relationship.id,
                vec![new_relationship.right_entity] as Vec<EntityId>,
            )?;
            created_relationships.push(new_relationship);

            if relationship.id == EntityId::default() {
                counter += 1;
            }
        }

        counter_table.insert("relationship".to_string(), counter)?;

        Ok(created_relationships)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Relationship>>, Error> {
        let mut relationships = Vec::new();
        let relationship_table = self.transaction.open_table(RELATIONSHIP_TABLE)?;
        let left_entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_RELATIONSHIP_LEFT_ENTITY_JUNCTION_TABLE)?;
        let right_entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_RELATIONSHIP_RIGHT_ENTITY_JUNCTION_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut relationship_iter = relationship_table.iter()?;
            let mut count = 0;

            while let Some(Ok((id, relationship_data))) = relationship_iter.next() {
                if count >= 1000 {
                    break;
                }

                let id = id.value();
                let relationship = relationship_data.value().clone();

                let left_entity = left_entity_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop()
                    .expect("relationship has no left entity");

                let right_entity = right_entity_junction_table
                    .get(&id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop()
                    .expect("relationship has no right entity");

                relationships.push(Some(Relationship {
                    left_entity,
                    right_entity,
                    ..relationship
                }));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let relationship = if let Some(guard) = relationship_table.get(id)? {
                    let relationship = guard.value().clone();
                    let left_entity = left_entity_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default()
                        .pop()
                        .expect("relationship has no left entity");

                    let right_entity = right_entity_junction_table
                        .get(id)?
                        .map(|guard| guard.value().clone())
                        .unwrap_or_default()
                        .pop()
                        .expect("relationship has no right entity");

                    Some(Relationship {
                        left_entity,
                        right_entity,
                        ..relationship
                    })
                } else {
                    None
                };
                relationships.push(relationship);
            }
        }

        Ok(relationships)
    }

    fn update_multi(&mut self, relationships: &[Relationship]) -> Result<Vec<Relationship>, Error> {
        let mut updated_relationships = Vec::new();
        let mut relationship_table = self.transaction.open_table(RELATIONSHIP_TABLE)?;
        let mut left_entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_RELATIONSHIP_LEFT_ENTITY_JUNCTION_TABLE)?;
        let mut right_entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_RELATIONSHIP_RIGHT_ENTITY_JUNCTION_TABLE)?;

        for relationship in relationships {
            relationship_table.insert(relationship.id, relationship)?;
            left_entity_junction_table.insert(
                relationship.id,
                vec![relationship.left_entity] as Vec<EntityId>,
            )?;
            right_entity_junction_table.insert(
                relationship.id,
                vec![relationship.right_entity] as Vec<EntityId>,
            )?;
            updated_relationships.push(relationship.clone());
        }

        Ok(updated_relationships)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut relationship_table = self.transaction.open_table(RELATIONSHIP_TABLE)?;
        let mut left_entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_RELATIONSHIP_LEFT_ENTITY_JUNCTION_TABLE)?;
        let mut right_entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_RELATIONSHIP_RIGHT_ENTITY_JUNCTION_TABLE)?;
        let mut relationship_from_entity_relationships_junction_table = self
            .transaction
            .open_table(RELATIONSHIP_FROM_ENTITY_RELATIONSHIPS_JUNCTION_TABLE)?;

        for id in ids {
            relationship_table.remove(id)?;
            left_entity_junction_table.remove(id)?;
            right_entity_junction_table.remove(id)?;
            db_helpers::delete_from_backward_junction_table(
                &mut relationship_from_entity_relationships_junction_table,
                id,
            )?;
        }

        Ok(())
    }
    fn get_relationship(
        &self,
        id: &EntityId,
        field: &RelationshipRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let guard = junction_table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()).unwrap_or_default())
    }

    fn get_relationships_from_right_ids(
        &self,
        field: &RelationshipRelationshipField,
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
        field: &RelationshipRelationshipField,
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
        field: &RelationshipRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        junction_table.insert(id.clone(), right_ids.to_vec())?;
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

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut relationship_iter = relationship_table.iter()?;
            let mut count = 0;

            while let Some(Ok((_, relationship_data))) = relationship_iter.next() {
                if count >= 1000 {
                    break;
                }

                relationships.push(Some(relationship_data.value().clone()));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let relationship = if let Some(guard) = relationship_table.get(id)? {
                    Some(guard.value().clone())
                } else {
                    None
                };
                relationships.push(relationship);
            }
        }

        Ok(relationships)
    }

    fn get_relationship(
        &self,
        id: &EntityId,
        field: &RelationshipRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        let junction_table_definition = get_junction_table_definition(field);
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let guard = junction_table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()).unwrap_or_default())
    }

    fn get_relationships_from_right_ids(
        &self,
        field: &RelationshipRelationshipField,
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
