use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::DtoField;
use crate::entities::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

use super::dto_field_repository::DtoFieldTable;
use super::dto_field_repository::DtoFieldTableRO;

const DTO_FIELD_TABLE: TableDefinition<EntityId, Bincode<DtoField>> =
    TableDefinition::new("dto_field");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// backward relationships
const DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("dto_field_from_dto_fields_junction");

pub struct DtoFieldRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> DtoFieldRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        DtoFieldRedbTable { transaction }
    }
}

impl<'a> DtoFieldTable for DtoFieldRedbTable<'a> {
    fn create(&mut self, dto_field: &DtoField) -> Result<DtoField, Error> {
        let dto_fields = self.create_multi(&[dto_field.clone()])?;
        Ok(dto_fields.into_iter().next().unwrap())
    }

    fn get(&self, id: &EntityId) -> Result<Option<DtoField>, Error> {
        let dto_fields = self.get_multi(&[id.clone()])?;
        Ok(dto_fields.into_iter().next().unwrap())
    }

    fn update(&mut self, dto_field: &DtoField) -> Result<DtoField, Error> {
        let dto_fields = self.update_multi(&[dto_field.clone()])?;
        Ok(dto_fields.into_iter().next().unwrap())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.delete_multi(&[id.clone()])
    }

    fn create_multi(&mut self, dto_fields: &[DtoField]) -> Result<Vec<DtoField>, Error> {
        let mut created_dto_fields = Vec::new();
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let mut counter = if let Some(counter) = counter_table.get(&"dto_field".to_string())? {
            counter.value()
        } else {
            1
        };

        let mut dto_field_table = self.transaction.open_table(DTO_FIELD_TABLE)?;

        for dto_field in dto_fields {
            // if the id is default, create a new id
            let new_dto_field = if dto_field.id == EntityId::default() {
                DtoField {
                    id: counter,
                    ..dto_field.clone()
                }
            } else {
                // ensure that the id is not already in use
                if dto_field_table.get(&dto_field.id)?.is_some() {
                    panic!("DtoField id already in use while creating it: {}", dto_field.id);
                }
                dto_field.clone()
            };
            dto_field_table.insert(new_dto_field.id, new_dto_field.clone())?;
            created_dto_fields.push(new_dto_field);
            counter += 1;
        }

        counter_table.insert("dto_field".to_string(), counter)?;

        Ok(created_dto_fields)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<DtoField>>, Error> {
        let mut dto_fields = Vec::new();
        let dto_field_table = self.transaction.open_table(DTO_FIELD_TABLE)?;

        for id in ids {
            let dto_field = if let Some(guard) = dto_field_table.get(id)? {
                Some(guard.value().clone())
            } else {
                None
            };
            dto_fields.push(dto_field);
        }
        Ok(dto_fields)
    }

    fn update_multi(&mut self, dto_fields: &[DtoField]) -> Result<Vec<DtoField>, Error> {
        let mut updated_dto_fields = Vec::new();
        let mut dto_field_table = self.transaction.open_table(DTO_FIELD_TABLE)?;

        for dto_field in dto_fields {
            dto_field_table.insert(dto_field.id, dto_field)?;
            updated_dto_fields.push(dto_field.clone());
        }

        Ok(updated_dto_fields)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut dto_field_table = self.transaction.open_table(DTO_FIELD_TABLE)?;
        let mut junction_table = self
            .transaction
            .open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;

        for id in ids {
            dto_field_table.remove(id)?;
            db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;
        }

        Ok(())
    }
}

pub struct DtoFieldRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> DtoFieldRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        DtoFieldRedbTableRO { transaction }
    }
}

impl<'a> DtoFieldTableRO for DtoFieldRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<DtoField>, Error> {
        let dto_fields = self.get_multi(&[id.clone()])?;
        Ok(dto_fields.into_iter().next().unwrap())
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<DtoField>>, Error> {
        let mut dto_fields = Vec::new();
        let dto_field_table = self.transaction.open_table(DTO_FIELD_TABLE)?;

        for id in ids {
            let dto_field = if let Some(guard) = dto_field_table.get(id)? {
                Some(guard.value().clone())
            } else {
                None
            };
            dto_fields.push(dto_field);
        }
        Ok(dto_fields)
    }
}
