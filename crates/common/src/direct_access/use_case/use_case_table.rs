use crate::database::Bincode;
use crate::entities::UseCase;
use crate::entities::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const USE_CASE_TABLE: TableDefinition<EntityId, Bincode<UseCase>> = TableDefinition::new("use_case");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
const ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("entity_from_use_case_entities_junction");
const DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("dto_from_use_case_dto_in_junction");
const DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("dto_from_use_case_dto_out_junction");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseCaseRelationshipField  {
    Entities,
    DtoIn,
    DtoOut,
}
pub trait UseCaseTable {
    fn create(&mut self, use_case: &UseCase) -> Result<UseCase, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error>;
    fn update(&mut self, use_case: &UseCase) -> Result<UseCase, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn get_relationships_of(&self, field: &UseCaseRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn delete_all_relationships_with(&mut self, field: &UseCaseRelationshipField, right_ids: &[EntityId]) -> Result<(), Error>;
    fn set_relationships(&mut self, field: &UseCaseRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<(), Error>;
}

pub trait UseCaseTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error>;
}

#[derive(Clone)]
pub struct UseCaseRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> UseCaseRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        UseCaseRedbTable {
            transaction,
        }
    }
}

impl<'a> UseCaseTable for UseCaseRedbTable<'a> {
    fn create(&mut self, use_case: &UseCase) -> Result<UseCase, Error> {
        // retrieve the counter
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let counter = if let Some(counter) = counter_table.get(&"use_case".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut table = self.transaction.open_table(USE_CASE_TABLE)?;

        let use_case = UseCase {
            id: counter,
            ..use_case.clone()
        };
        table.insert(use_case.id, use_case.clone())?;

        // update the counter
        counter_table.insert("use_case".to_string(), counter)?;

        // add entities to junction table
        let mut junction_table = self.transaction.open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        junction_table.insert(use_case.id, use_case.entities.clone())?;

        // add dto_in to junction table
        let mut dto_in_junction_table = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        dto_in_junction_table.insert(use_case.id, use_case.dto_in.clone().into_iter().collect::<Vec<EntityId>>())?;

        // add dto_out to junction table
        let mut dto_out_junction_table = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;
        dto_out_junction_table.insert(use_case.id, use_case.dto_out.clone().into_iter().collect::<Vec<EntityId>>())?;

        Ok(use_case)
    }

    fn get(&self, id: &EntityId) -> Result<Option<UseCase>, Error> {
        let table = self.transaction.open_table(USE_CASE_TABLE)?;
        let guard = table.get(id)?;
        let use_case = guard.map(|guard| guard.value().clone());

        if use_case.is_none() {
            return Ok(None);
        }
        
        // get entities from junction table
        let entities_junction_table = self.transaction.open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        let entities = entities_junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default();

        // get dto_in from junction table
        let dto_in_junction_table = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        let dto_in: Option<EntityId> = dto_in_junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default().pop();

        // get dto_out from junction table
        let dto_out_junction_table = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;
        let dto_out: Option<EntityId> = dto_out_junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default().pop();


        Ok(use_case.map(|mut use_case| {
            use_case.entities = entities;
            use_case.dto_in = dto_in;
            use_case.dto_out = dto_out;            
            use_case
        }))
    }

    fn update(&mut self, use_case: &UseCase) -> Result<UseCase, Error> {
        // update the use case table
        let mut table = self.transaction.open_table(USE_CASE_TABLE)?;
        table.insert(use_case.id, use_case)?;

        // update the junction table
        let mut junction_table = self.transaction.open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        junction_table.insert(use_case.id, use_case.entities.clone())?;

        // update the dto_in junction table
        let mut dto_in_junction_table = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        dto_in_junction_table.insert(use_case.id, use_case.dto_in.clone().into_iter().collect::<Vec<EntityId>>())?;

        // update the dto_out junction table
        let mut dto_out_junction_table = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;
        dto_out_junction_table.insert(use_case.id, use_case.dto_out.clone().into_iter().collect::<Vec<EntityId>>())?;

        Ok(use_case.clone())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        // delete from use case table
        let mut table = self.transaction.open_table(USE_CASE_TABLE)?;
        table.remove(id)?;

        // delete from junction table
        let mut junction_table = self.transaction.open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        junction_table.remove(id)?;

        // delete from junction table
        let mut dto_in_junction_table = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        dto_in_junction_table.remove(id)?;

        // delete from junction table
        let mut dto_out_junction_table = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;
        dto_out_junction_table.remove(id)?;

        Ok(())
    }

    fn get_relationships_of(&self, field: &UseCaseRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition =
            match field {
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
            if right_ids.iter().any(|entity_id| right_entities.contains(entity_id)) {
                relationships.push((left_id, right_entities));
            }
        }
        Ok(relationships)
    }

    fn delete_all_relationships_with(&mut self, field: &UseCaseRelationshipField, right_ids: &[EntityId]) -> Result<(), Error> {
        // delete from junction table        
        let junction_table_definition =
            match field {
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

    fn set_relationships(&mut self, field: &UseCaseRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<(), Error> {
        let junction_table_definition =
            match field {
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
        let table = self.transaction.open_table(USE_CASE_TABLE)?;
        let guard = table.get(id)?; 
        let use_case = guard.map(|guard| guard.value().clone());

        if use_case.is_none() {
            return Ok(None);
        }

        // get entities from junction table
        let entities_junction_table = self.transaction.open_table(ENTITY_FROM_USE_CASE_ENTITIES_JUNCTION_TABLE)?;
        let entities = entities_junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default();

        // get dto_in from junction table
        let dto_in_junction_table = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        let dto_in: Option<EntityId> = dto_in_junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default().pop();
        
        // get dto_out from junction table
        let dto_out_junction_table = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;
        let dto_out: Option<EntityId> = dto_out_junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default().pop();


        Ok(use_case.map(|mut use_case| {
            use_case.entities = entities;
            use_case.dto_in = dto_in;
            use_case.dto_out = dto_out;            
            use_case
        }))
    }
}
