use crate::direct_access::{DbConnectionTrait, RepositoryError};
use anyhow::Result;
use std::path::Path;
use std::any::Any;
use super::DatabaseError;

pub struct SqliteDbConnection<'a> {
    pub connection: rusqlite::Connection,
    transaction: Option<rusqlite::Transaction<'a>>,
}

impl<'a> SqliteDbConnection<'a> {
    pub fn new(database_path: &Path) -> Result<SqliteDbConnection<'a>, DatabaseError> {
        let connection = rusqlite::Connection::open(database_path)?;
        Ok(SqliteDbConnection { connection, transaction: None })
    }

}

impl<'a> DbConnectionTrait for SqliteDbConnection<'a> {
    fn begin_transaction(&mut self) -> Result<(), RepositoryError> {
        self.transaction = Some(self.connection.transaction().map_err(DatabaseError::from)?);
        Ok(())
    }

    fn commit(&self) -> Result<(), RepositoryError> {
        todo!()
    }

    fn rollback(&self) -> Result<(), RepositoryError> {
        todo!()
    }
}
