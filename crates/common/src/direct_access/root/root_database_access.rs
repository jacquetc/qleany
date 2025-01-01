use std::rc::Rc;
use crate::direct_access::root::root_repository::RootDatabaseAccessTrait;
use crate::direct_access::root::Root;
use crate::database::DatabaseError;
use crate::database::sqlite_db_connection::SqliteDbConnection;
use crate::database::{DatabaseAccessTrait, DbContextTrait};
use crate::direct_access::{DbConnectionTrait};

pub(crate) struct RootDatabaseAccess {
    db_connection: Rc<dyn DbConnectionTrait>,
}

impl RootDatabaseAccess {
    pub fn new(db_connection: Rc<dyn DbConnectionTrait>) -> RootDatabaseAccess {
        RootDatabaseAccess { db_connection }
    }
}


impl RootDatabaseAccessTrait for RootDatabaseAccess {}

impl DatabaseAccessTrait<Root> for RootDatabaseAccess {

    fn create(&self, entities: &[Root]) -> Result<Vec<Root>, DatabaseError> {
        self.db_connection
    }
    fn get(&self, ids: &[i64]) -> Result<Vec<Root>, DatabaseError> {
        
        Ok(vec![])
    }
    fn get(&self, ids: &[i64]) -> Result<Vec<Root>, DatabaseError> {

    }

    fn update(&self, entities: &[Root]) -> Result<Vec<Root>, DatabaseError> {

    }

    fn remove(&self, ids: &[i64]) -> Result<(), DatabaseError> {

    }
    
}
