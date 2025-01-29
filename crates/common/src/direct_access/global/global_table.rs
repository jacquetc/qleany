use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::Global;
use crate::entities::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

const GLOBAL_TABLE: TableDefinition<EntityId, Bincode<Global>> = TableDefinition::new("global");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// backward relationships
const GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> = TableDefinition::new("global_from_root_global_junction");

pub trait GlobalTable {
    fn create(&mut self, global: &Global) -> Result<Global, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Global>, Error>;
    fn update(&mut self, global: &Global) -> Result<Global, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
}

pub trait GlobalTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Global>, Error>;
}

#[derive(Clone)]
pub struct GlobalRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> GlobalRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        GlobalRedbTable {
            transaction,
        }
    }
}

impl<'a> GlobalTable for GlobalRedbTable<'a> {
    fn create(&mut self, global: &Global) -> Result<Global, Error> {
        // retrieve the counter
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let counter = if let Some(counter) = counter_table.get(&"global".to_string())? {
            counter.value() + 1
        } else {
            1
        };

        let mut table = self.transaction.open_table(GLOBAL_TABLE)?;

        let global = Global {
            id: counter,
            ..global.clone()
        };
        table.insert(global.id, global.clone())?;

        // update the counter
        counter_table.insert("global".to_string(), counter)?;

        Ok(global)
    }

    fn get(&self, id: &EntityId) -> Result<Option<Global>, Error> {
        let table = self.transaction.open_table(GLOBAL_TABLE)?;
        let guard = table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()))
    }

    fn update(&mut self, global: &Global) -> Result<Global, Error> {
        let mut table = self.transaction.open_table(GLOBAL_TABLE)?;
        table.insert(global.id, global)?;
        Ok(global.clone())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        let mut table = self.transaction.open_table(GLOBAL_TABLE)?;
        table.remove(id)?;

        // delete from backward junction tables, where the id may be in the Vec in the value
        let mut junction_table: redb::Table<'_, u64, Vec<u64>> = self.transaction.open_table(GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE)?;
        db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;

        Ok(())
    }
}

pub struct GlobalRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> GlobalRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        GlobalRedbTableRO { transaction }
    }
}

impl<'a> GlobalTableRO for GlobalRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<Global>, Error> {
        let table = self.transaction.open_table(GLOBAL_TABLE)?;
        let guard = table.get(id)?;
        Ok(guard.map(|guard| guard.value().clone()))
    }
}
