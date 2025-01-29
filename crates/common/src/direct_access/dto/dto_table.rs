use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::Dto;
use crate::entities::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const DTO_TABLE: TableDefinition<EntityId, Bincode<Dto>> = TableDefinition::new("dto");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("dto_field_from_dto_fields_junction");
// backward relationships
const DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("dto_from_use_case_dto_in_junction");
const DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("dto_from_use_case_dto_out_junction");


pub enum DtoRelationshipField {
    Fields
}

pub trait DtoTable {
    fn create(&mut self, dto: &Dto) -> Result<Dto, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error>;
    fn update(&mut self, dto: &Dto) -> Result<Dto, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn get_relationships_of(&self, field: &DtoRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn delete_all_relationships_with(&mut self, field: &DtoRelationshipField, right_ids: &[EntityId]) -> Result<(), Error>;
    fn set_relationships(&mut self, field: &DtoRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<(), Error>;

}

pub trait DtoTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error>;
}

#[derive(Clone)]
pub struct DtoRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> DtoRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        DtoRedbTable { transaction }
    }
}

impl<'a> DtoTable for DtoRedbTable<'a> {
    fn create(&mut self, dto: &Dto) -> Result<Dto, Error> {
        // retrieve the counter
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let counter = if let Some(counter) = counter_table.get(&"feature".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut table = self.transaction.open_table(DTO_TABLE)?;

        let new_dto = Dto {
            id: counter,
            ..dto.clone()
        };
        table.insert(new_dto.id, new_dto.clone())?;

        // update the counter
        counter_table.insert("feature".to_string(), counter)?;

        // add use cases to junction table
        let mut junction_table = self.transaction.open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;
        junction_table.insert(new_dto.id, new_dto.fields.clone())?;

        Ok(new_dto)
    }

    fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error> {
        let table = self.transaction.open_table(DTO_TABLE)?;
        let guard = table.get(id)?;
        let dto = guard.map(|guard| guard.value().clone());

        if dto.is_none() {
            return Ok(None);
        }

        // get fields from junction table
        let junction_table = self.transaction.open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;
        let fields = junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default();

        Ok(dto.map(|mut dto| {
            dto.fields = fields;
            dto
        }))
    }

    fn update(&mut self, dto: &Dto) -> Result<Dto, Error> {
        // update the dto table
        let mut table = self.transaction.open_table(DTO_TABLE)?;
        table.insert(dto.id, dto)?;

        // update the junction table
        let mut junction_table = self.transaction.open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;
        junction_table.insert(dto.id, dto.fields.clone())?;

        Ok(dto.clone())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        // delete from dto table
        let mut table = self.transaction.open_table(DTO_TABLE)?;
        table.remove(id)?;

        // delete from junction table
        let mut junction_table = self.transaction.open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;
        junction_table.remove(id)?;

        // delete from backward junction tables, where the id may be in the Vec in the value
        let mut junction_table: redb::Table<'_, u64, Vec<u64>> = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;

        let mut junction_table: redb::Table<'_, u64, Vec<u64>> = self.transaction.open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;
        db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;

        Ok(())


    }
    
    fn get_relationships_of(&self, field: &DtoRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition =
            match field {
                DtoRelationshipField::Fields => DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE,
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
    
    fn delete_all_relationships_with(&mut self, field: &DtoRelationshipField, right_ids: &[EntityId]) -> Result<(), Error> {
        // delete from junction table        
        let junction_table_definition =
            match field {
                DtoRelationshipField::Fields => DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE,
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
    
    fn set_relationships(&mut self, field: &DtoRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<(), Error> {
        let junction_table_definition =
            match field {
                DtoRelationshipField::Fields => DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE,
        };
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        for (left_id, entities) in relationships {
            junction_table.insert(left_id, entities)?;
        }
        Ok(())
    }
}

pub struct DtoRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> DtoRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        DtoRedbTableRO { transaction }
    }
}

impl<'a> DtoTableRO for DtoRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error> {
        let table = self.transaction.open_table(DTO_TABLE)?;
        let guard = table.get(id)?;
        let dto = guard.map(|guard| guard.value().clone());

        if dto.is_none() {
            return Ok(None);
        }

        // get fields from junction table
        let junction_table = self.transaction.open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;
        let fields = junction_table.get(id)?.map(|guard| guard.value().clone()).unwrap_or_default();

        Ok(dto.map(|mut dto| {
            dto.fields = fields;
            dto
        }))
    }
}
