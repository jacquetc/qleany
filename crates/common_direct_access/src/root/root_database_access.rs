use std::rc::Rc;
use crate::root::root_repository::RootDatabaseAccessTrait;
use common_entities::root::Root;
use common_persistence::database::sqlite_database_access::SqliteDatabaseAccess;
use common_persistence::database::DatabaseError;
use common_persistence::database::{DatabaseAccessTrait, DbContextTrait};
use direct_access::DbConnectionTrait;

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
        SqliteDatabaseAccess::<Root>::create(db_connection, entities)
    }

    fn get(&self, ids: &[i64]) -> Result<Vec<Root>, DatabaseError> {
        SqliteDatabaseAccess::<Root>::get(db_connection, ids)
    }

    fn update(&self, entities: &[Root]) -> Result<Vec<Root>, DatabaseError> {
        SqliteDatabaseAccess::<Root>::update(db_connection, entities)
    }

    fn remove(&self, ids: &[i64]) -> Result<(), DatabaseError> {
        SqliteDatabaseAccess::<Root>::remove(db_connection, ids)
    }
}
