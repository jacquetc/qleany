use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::File;
use crate::types::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

use super::file_repository::FileTable;
use super::file_repository::FileTableRO;

const FILE_TABLE: TableDefinition<EntityId, Bincode<File>> = TableDefinition::new("file");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// backward relationships
const FILE_FROM_ROOT_FILE_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("file_from_root_file_junction");

pub struct FileRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> FileRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        FileRedbTable { transaction }
    }

    pub fn init_tables(transaction: &WriteTransaction) -> Result<(), Error> {
        transaction.open_table(FILE_TABLE)?;
        transaction.open_table(COUNTER_TABLE)?;
        transaction.open_table(FILE_FROM_ROOT_FILE_JUNCTION_TABLE)?;
        Ok(())
    }
}

impl<'a> FileTable for FileRedbTable<'a> {
    fn create(&mut self, file: &File) -> Result<File, Error> {
        let files = self.create_multi(&[file.clone()])?;
        Ok(files.into_iter().next().unwrap())
    }

    fn get(&self, id: &EntityId) -> Result<Option<File>, Error> {
        let files = self.get_multi(&[id.clone()])?;
        Ok(files.into_iter().next().unwrap())
    }

    fn update(&mut self, file: &File) -> Result<File, Error> {
        let files = self.update_multi(&[file.clone()])?;
        Ok(files.into_iter().next().unwrap())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.delete_multi(&[id.clone()])
    }

    fn create_multi(&mut self, files: &[File]) -> Result<Vec<File>, Error> {
        let mut created_files = Vec::new();
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let mut counter = if let Some(counter) = counter_table.get(&"file".to_string())? {
            counter.value()
        } else {
            1
        };

        let mut file_table = self.transaction.open_table(FILE_TABLE)?;

        for file in files {
            // id the id is default, create a new id
            let new_file = if file.id == EntityId::default() {
                File {
                    id: counter,
                    ..file.clone()
                }
            } else {
                // ensure that the id is not already in use
                if file_table.get(&file.id)?.is_some() {
                    panic!("File id already in use while creating it: {:?}", file.id);
                }
                file.clone()
            };
            file_table.insert(new_file.id, new_file.clone())?;
            created_files.push(new_file);

            if file.id == EntityId::default() {
                counter += 1;
            }
        }

        counter_table.insert("file".to_string(), counter)?;

        Ok(created_files)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<File>>, Error> {
        let mut files = Vec::new();
        let file_table = self.transaction.open_table(FILE_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut file_iter = file_table.iter()?;
            let mut count = 0;

            while let Some(Ok((_, file_data))) = file_iter.next() {
                if count >= 1000 {
                    break;
                }

                files.push(Some(file_data.value().clone()));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let file = if let Some(guard) = file_table.get(id)? {
                    Some(guard.value().clone())
                } else {
                    None
                };
                files.push(file);
            }
        }

        Ok(files)
    }

    fn update_multi(&mut self, files: &[File]) -> Result<Vec<File>, Error> {
        let mut updated_files = Vec::new();
        let mut file_table = self.transaction.open_table(FILE_TABLE)?;

        for file in files {
            file_table.insert(file.id, file)?;
            updated_files.push(file.clone());
        }

        Ok(updated_files)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut file_table = self.transaction.open_table(FILE_TABLE)?;
        let mut junction_table = self
            .transaction
            .open_table(FILE_FROM_ROOT_FILE_JUNCTION_TABLE)?;

        for id in ids {
            file_table.remove(id)?;
            db_helpers::delete_from_backward_junction_table(&mut junction_table, id)?;
        }

        Ok(())
    }
}

pub struct FileRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> FileRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        FileRedbTableRO { transaction }
    }
}

impl<'a> FileTableRO for FileRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<File>, Error> {
        let files = self.get_multi(&[id.clone()])?;
        Ok(files.into_iter().next().unwrap())
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<File>>, Error> {
        let mut files = Vec::new();
        let file_table = self.transaction.open_table(FILE_TABLE)?;

        // If ids is empty, return all entities (up to 1000)
        if ids.is_empty() {
            let mut file_iter = file_table.iter()?;
            let mut count = 0;

            while let Some(Ok((_, file_data))) = file_iter.next() {
                if count >= 1000 {
                    break;
                }

                files.push(Some(file_data.value().clone()));
                count += 1;
            }
        } else {
            // Original behavior for non-empty ids
            for id in ids {
                let file = if let Some(guard) = file_table.get(id)? {
                    Some(guard.value().clone())
                } else {
                    None
                };
                files.push(file);
            }
        }

        Ok(files)
    }
}
