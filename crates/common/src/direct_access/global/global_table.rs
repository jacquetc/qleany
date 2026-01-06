use crate::database::Bincode;
use crate::database::db_helpers;
use crate::entities::Global;
use crate::types::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

use super::global_repository::GlobalTable;
use super::global_repository::GlobalTableRO;

const GLOBAL_TABLE: TableDefinition<EntityId, Bincode<Global>> = TableDefinition::new("global");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// backward relationships
const GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("global_from_root_global_junction");

pub struct GlobalRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> GlobalRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        GlobalRedbTable { transaction }
    }

    pub fn init_tables(transaction: &WriteTransaction) -> Result<(), Error> {
        transaction.open_table(GLOBAL_TABLE)?;
        transaction.open_table(COUNTER_TABLE)?;
        transaction.open_table(GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE)?;
        Ok(())
    }
}

impl<'a> GlobalTable for GlobalRedbTable<'a> {
    fn create(&mut self, global: &Global) -> Result<Global, Error> {
        let globals = self.create_multi(&[global.clone()])?;
        Ok(globals.into_iter().next().unwrap())
    }

    fn get(&self, id: &EntityId) -> Result<Option<Global>, Error> {
        let globals = self.get_multi(&[id.clone()])?;
        Ok(globals.into_iter().next().unwrap())
    }

    fn update(&mut self, global: &Global) -> Result<Global, Error> {
        let globals = self.update_multi(&[global.clone()])?;
        Ok(globals.into_iter().next().unwrap())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.delete_multi(&[id.clone()])
    }

    fn create_multi(&mut self, globals: &[Global]) -> Result<Vec<Global>, Error> {
        let mut created_globals = Vec::new();
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let mut counter = if let Some(counter) = counter_table.get(&"global".to_string())? {
            counter.value()
        } else {
            1
        };

        let mut global_table = self.transaction.open_table(GLOBAL_TABLE)?;

        for global in globals {
            // id the id is default, create a new id
            let new_global = if global.id == EntityId::default() {
                Global {
                    id: counter,
                    ..global.clone()
                }
            } else {
                // ensure that the id is not already in use
                if global_table.get(&global.id)?.is_some() {
                    panic!(
                        "Global id already in use while creating it: {:?}",
                        global.id
                    );
                }
                global.clone()
            };
            global_table.insert(new_global.id, new_global.clone())?;
            created_globals.push(new_global);

            if global.id == EntityId::default() {
                counter += 1;
            }
        }

        counter_table.insert("global".to_string(), counter)?;

        Ok(created_globals)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Global>>, Error> {
        let mut globals = Vec::new();
        let global_table = self.transaction.open_table(GLOBAL_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut global_iter = global_table.iter()?;
            let mut count = 0;

            while let Some(Ok((_, global_data))) = global_iter.next() {
                if count >= 1000 {
                    break;
                }

                globals.push(Some(global_data.value().clone()));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let global = if let Some(guard) = global_table.get(id)? {
                    Some(guard.value().clone())
                } else {
                    None
                };
                globals.push(global);
            }
        }

        Ok(globals)
    }

    fn update_multi(&mut self, globals: &[Global]) -> Result<Vec<Global>, Error> {
        let mut updated_globals = Vec::new();
        let mut global_table = self.transaction.open_table(GLOBAL_TABLE)?;

        for global in globals {
            global_table.insert(global.id, global)?;
            updated_globals.push(global.clone());
        }

        Ok(updated_globals)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut global_table = self.transaction.open_table(GLOBAL_TABLE)?;
        let mut junction_table = self
            .transaction
            .open_table(GLOBAL_FROM_ROOT_GLOBAL_JUNCTION_TABLE)?;

        for id in ids {
            global_table.remove(id)?;
            db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;
        }

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
        let globals = self.get_multi(&[id.clone()])?;
        Ok(globals.into_iter().next().unwrap())
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Global>>, Error> {
        let mut globals = Vec::new();
        let global_table = self.transaction.open_table(GLOBAL_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut global_iter = global_table.iter()?;
            let mut count = 0;

            while let Some(Ok((_, global_data))) = global_iter.next() {
                if count >= 1000 {
                    break;
                }

                globals.push(Some(global_data.value().clone()));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let global = if let Some(guard) = global_table.get(id)? {
                    Some(guard.value().clone())
                } else {
                    None
                };
                globals.push(global);
            }
        }

        Ok(globals)
    }
}
