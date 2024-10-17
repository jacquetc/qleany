pub mod sqlite_database_access;

use direct_access::RepositoryError;
use thiserror::Error;


pub trait DbContextTrait : Send + Sync + 'static  {
    fn get_connection(&self) -> &str;
}

pub trait DatabaseAccessTrait<T> {
    fn create(&self, entities: &[T]) -> Result<Vec<T>, DatabaseError>;
    fn get(&self, id: &[i64]) -> Result<Vec<T>, DatabaseError>;
    fn update(&self, entities: &[T]) -> Result<Vec<T>, DatabaseError>;
    fn remove(&self, id: &[i64])-> Result<(), DatabaseError>;
}


#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Entity not found")]
    NotFound,
    #[error("Entity already exists")]
    AlreadyExists,
    #[error("Database error")]
    DatabaseError()
}

impl From<DatabaseError> for RepositoryError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::NotFound => RepositoryError::NotFound,
            DatabaseError::AlreadyExists => RepositoryError::AlreadyExists,
            DatabaseError::DatabaseError() => RepositoryError::DatabaseError(),
        }
    }
}
