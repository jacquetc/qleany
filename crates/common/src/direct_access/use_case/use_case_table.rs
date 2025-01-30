use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::EntityId;
use crate::entities::UseCase;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

use super::use_case_repository::UseCaseTable;
use super::use_case_repository::UseCaseTableRO;
use super::UseCaseRelationshipField;

const USE_CASE_TABLE: TableDefinition<EntityId, Bincode<UseCase>> =
    TableDefinition::new("use_case");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("entity_from_use_case_entities_junction");
const DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("dto_from_use_case_dto_in_junction");
const DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("dto_from_use_case_dto_out_junction");
// backward relationships
const USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("use_case_from_feature_use_cases_junction");

pub struct UseCaseRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> UseCaseRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        UseCaseRedbTable { transaction }
    }
}

impl<'a> UseCaseTable for UseCaseRedbTable<'a> {
    fn create(&mut self, use_case: &UseCase) -> Result<UseCase, Error> {
        let use_cases = self.create_multi(&[use_case.clone()])?;
        Ok(use_cases.into_iter().next().unwrap())
    }

    fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error> {
        let use_cases = self.get_multi(&[id.clone()])?;
        Ok(use_cases.into_iter().next().unwrap())
    }

    fn update(&mut self, use_case: &UseCase) -> Result<UseCase, Error> {
        let use_cases = self.update_multi(&[use_case.clone()])?;
        Ok(use_cases.into_iter().next().unwrap())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.delete_multi(&[id.clone()])
    }

    fn create_multi(&mut self, use_cases: &[UseCase]) -> Result<Vec<UseCase>, Error> {
        let mut created_use_cases = Vec::new();
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let mut counter = if let Some(counter) = counter_table.get(&"use_case".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut use_case_table = self.transaction.open_table(USE_CASE_TABLE)?;
        let mut entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        let mut dto_in_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        let mut dto_out_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;

        for use_case in use_cases {
            // if the id is default, create a new id
            let new_use_case = if use_case.id == EntityId::default() {
                UseCase {
                    id: counter,
                    ..use_case.clone()
                }
            } else {
                // ensure that the id is not already in use
                if use_case_table.get(&use_case.id)?.is_some() {
                    panic!(
                        "UseCase id already in use while creating it: {:?}",
                        use_case.id
                    );
                }
                use_case.clone()
            };
            use_case_table.insert(new_use_case.id, new_use_case.clone())?;
            entity_junction_table.insert(new_use_case.id, new_use_case.entities.clone())?;
            dto_in_junction_table.insert(
                new_use_case.id,
                new_use_case
                    .dto_in
                    .clone()
                    .into_iter()
                    .collect::<Vec<EntityId>>(),
            )?;
            dto_out_junction_table.insert(
                new_use_case.id,
                new_use_case
                    .dto_out
                    .clone()
                    .into_iter()
                    .collect::<Vec<EntityId>>(),
            )?;
            created_use_cases.push(new_use_case);

            if use_case.id == EntityId::default() {
                counter += 1;
            }
        }

        counter_table.insert("use_case".to_string(), counter)?;

        Ok(created_use_cases)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>, Error> {
        let mut use_cases = Vec::new();
        let use_case_table = self.transaction.open_table(USE_CASE_TABLE)?;
        let entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        let dto_in_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        let dto_out_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;

        for id in ids {
            let use_case = if let Some(guard) = use_case_table.get(id)? {
                let mut use_case = guard.value().clone();

                // get entities from junction table
                let entities = entity_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                // get dto_in from junction table
                let dto_in: Option<EntityId> = dto_in_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop();

                // get dto_out from junction table
                let dto_out: Option<EntityId> = dto_out_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop();

                use_case.entities = entities;
                use_case.dto_in = dto_in;
                use_case.dto_out = dto_out;
                Some(use_case)
            } else {
                None
            };
            use_cases.push(use_case);
        }
        Ok(use_cases)
    }

    fn update_multi(&mut self, use_cases: &[UseCase]) -> Result<Vec<UseCase>, Error> {
        let mut updated_use_cases = Vec::new();
        let mut use_case_table = self.transaction.open_table(USE_CASE_TABLE)?;
        let mut entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        let mut dto_in_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        let mut dto_out_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;

        for use_case in use_cases {
            use_case_table.insert(use_case.id, use_case)?;
            entity_junction_table.insert(use_case.id, use_case.entities.clone())?;
            dto_in_junction_table.insert(
                use_case.id,
                use_case
                    .dto_in
                    .clone()
                    .into_iter()
                    .collect::<Vec<EntityId>>(),
            )?;
            dto_out_junction_table.insert(
                use_case.id,
                use_case
                    .dto_out
                    .clone()
                    .into_iter()
                    .collect::<Vec<EntityId>>(),
            )?;
            updated_use_cases.push(use_case.clone());
        }

        Ok(updated_use_cases)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut use_case_table = self.transaction.open_table(USE_CASE_TABLE)?;
        let mut entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        let mut dto_in_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        let mut dto_out_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;
        let mut use_case_from_feature_use_cases_junction_table = self
            .transaction
            .open_table(USE_CASE_FROM_FEATURE_USE_CASES_JUNCTION_TABLE)?;

        for id in ids {
            use_case_table.remove(id)?;
            entity_junction_table.remove(id)?;
            dto_in_junction_table.remove(id)?;
            dto_out_junction_table.remove(id)?;
            db_helpers::delete_from_backward_junction_table(
                &mut use_case_from_feature_use_cases_junction_table,
                id,
            )?;
        }

        Ok(())
    }

    fn get_relationships_of(
        &self,
        field: &UseCaseRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition = match field {
            UseCaseRelationshipField::Entities => ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE,
            UseCaseRelationshipField::DtoIn => DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE,
            UseCaseRelationshipField::DtoOut => DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE,
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
        field: &UseCaseRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        // delete from junction table
        let junction_table_definition = match field {
            UseCaseRelationshipField::Entities => ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE,
            UseCaseRelationshipField::DtoIn => DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE,
            UseCaseRelationshipField::DtoOut => DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE,
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
        field: &UseCaseRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        let junction_table_definition = match field {
            UseCaseRelationshipField::Entities => ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE,
            UseCaseRelationshipField::DtoIn => DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE,
            UseCaseRelationshipField::DtoOut => DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE,
        };
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        for (left_id, entities) in relationships {
            junction_table.insert(left_id, entities)?;
        }
        Ok(())
    }
}

pub struct UseCaseRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> UseCaseRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        UseCaseRedbTableRO { transaction }
    }
}

impl<'a> UseCaseTableRO for UseCaseRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error> {
        let use_cases = self.get_multi(&[id.clone()])?;
        Ok(use_cases.into_iter().next().unwrap())
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>, Error> {
        let mut use_cases = Vec::new();
        let use_case_table = self.transaction.open_table(USE_CASE_TABLE)?;
        let entity_junction_table = self
            .transaction
            .open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        let dto_in_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        let dto_out_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;

        for id in ids {
            let use_case = if let Some(guard) = use_case_table.get(id)? {
                let mut use_case = guard.value().clone();

                // get entities from junction table
                let entities = entity_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                // get dto_in from junction table
                let dto_in: Option<EntityId> = dto_in_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop();

                // get dto_out from junction table
                let dto_out: Option<EntityId> = dto_out_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default()
                    .pop();

                use_case.entities = entities;
                use_case.dto_in = dto_in;
                use_case.dto_out = dto_out;
                Some(use_case)
            } else {
                None
            };
            use_cases.push(use_case);
        }
        Ok(use_cases)
    }
}
