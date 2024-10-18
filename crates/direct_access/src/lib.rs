pub mod root;

use root::RootRepositoryTrait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Entity not found")]
    NotFound,
    #[error("Entity already exists")]
    AlreadyExists,
    #[error("Database error")]
    DatabaseError(),
}

pub trait DbConnectionTrait {

    fn begin_transaction(&self) -> Result<(), RepositoryError>;
    fn commit(&self) -> Result<(), RepositoryError>;
    fn rollback(&self) -> Result<(), RepositoryError>;
}
pub trait RepositoryTrait<T> {
    fn create(&self, entities: &[T]) -> Result<Vec<T>, RepositoryError>;
    fn get(&self, id: &[i64]) -> Result<Vec<T>, RepositoryError>;
    fn update(&self, entities: &[T]) -> Result<Vec<T>, RepositoryError>;
    fn remove(&self, id: &[i64])-> Result<(), RepositoryError>;
}

pub trait RepositoryFactoryTrait {
    fn get_root_repository(&self, db_connection: &impl DbConnectionTrait) -> Box<dyn RootRepositoryTrait>;
}