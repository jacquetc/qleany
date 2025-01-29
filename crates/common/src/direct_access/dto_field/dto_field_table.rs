use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::DtoField;
use crate::entities::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const DTO_FIELD_TABLE: TableDefinition<EntityId, Bincode<DtoField>> = TableDefinition::new("dto_field");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// backward relationships
const DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("dto_field_from_dto_fields_junction");

pub trait DtoFieldTable {
    fn create(&mut self, dto_field: &DtoField) -> Result<DtoField, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<DtoField>, Error>;
    fn update(&mut self, dto_field: &DtoField) -> Result<DtoField, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
}

pub trait DtoFieldTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<DtoField>, Error>;
}

#[derive(Clone)]
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
        // retrieve the counter
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let counter = if let Some(counter) = counter_table.get(&"global".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut table = self.transaction.open_table(DTO_FIELD_TABLE)?;

        let dto_field = DtoField {
            id: counter,
            ..dto_field.clone()
        };
        table.insert(dto_field.id, dto_field.clone())?;

        // update the counter
        counter_table.insert("dto_field".to_string(), counter)?;

        Ok(dto_field)
    }

    fn get(&self, id: &EntityId) -> Result<Option<DtoField>, Error> {
        let table = self.transaction.open_table(DTO_FIELD_TABLE)?;
        let guard = table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()))
    }

    fn update(&mut self, dto_field: &DtoField) -> Result<DtoField, Error> {
        let mut table = self.transaction.open_table(DTO_FIELD_TABLE)?;
        table.insert(dto_field.id, dto_field.clone())?;
        Ok(dto_field.clone())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        let mut table = self.transaction.open_table(DTO_FIELD_TABLE)?;
        table.remove(id)?;

        // delete from backward junction tables, where the id may be in the Vec in the value
        let mut junction_table: redb::Table<'_, u64, Vec<u64>> = self.transaction.open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;
        db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;

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
        let table = self.transaction.open_table(DTO_FIELD_TABLE)?;
        let guard = table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()))
    }
}
