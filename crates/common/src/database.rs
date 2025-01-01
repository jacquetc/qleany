pub mod sqlite_db_context;
pub mod sqlite_db_connection;
use std::any::Any;

use crate::direct_access::{DbConnectionTrait, RepositoryError};
use sqlite_db_connection::SqliteDbConnection;
use thiserror::Error;

pub trait DbContextTrait{
    fn create_connection(&self) -> Result<impl DbConnectionTrait, DatabaseError>;
}


pub trait DatabaseAccessTrait<T> : Any {
    fn create(&self, entities: &[T]) -> Result<Vec<T>, DatabaseError>;
    fn get(&self, id: &[i64]) -> Result<Vec<T>, DatabaseError>;
    fn update(&self, entities: &[T]) -> Result<Vec<T>, DatabaseError>;
    fn remove(&self, id: &[i64]) -> Result<(), DatabaseError>;
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Entity not found")]
    NotFound,
    #[error("Entity already exists")]
    AlreadyExists,
    #[error("Database error")]
    DatabaseError(#[from] rusqlite::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<DatabaseError> for RepositoryError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::NotFound => RepositoryError::NotFound,
            DatabaseError::AlreadyExists => RepositoryError::AlreadyExists,
            DatabaseError::DatabaseError(e) => RepositoryError::DatabaseError(e.to_string()),
            DatabaseError::Other(_) => todo!(),
        }
    }
}
