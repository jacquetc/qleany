use common_entities::root::Root;
use common_persistence::database::{sqlite_database_access::SqliteDatabaseAccess, DatabaseAccessTrait, DbContextTrait};
use crate::root::root_repository::RootDatabaseAccessTrait;
use common_persistence::database::DatabaseError;


pub(crate) struct RootDatabaseAccess {
    db_context: Box<dyn DbContextTrait>,
}

impl RootDatabaseAccess {
    pub fn new(db_context: Box<dyn DbContextTrait>) -> RootDatabaseAccess {
        RootDatabaseAccess {
            db_context ,
        }
    }
}

impl RootDatabaseAccessTrait for RootDatabaseAccess {}

impl DatabaseAccessTrait<Root> for RootDatabaseAccess {
    fn create(&self, entities: &[Root]) -> Result<Vec<Root>, DatabaseError> {
        SqliteDatabaseAccess::<Root>create(entities)
    }

    fn get(&self, id: &[i64]) -> Result<Vec<Root>, DatabaseError> {
        todo!()
    }

    fn update(&self, entities: &[Root]) -> Result<Vec<Root>, DatabaseError> {
        todo!()
    }

    fn remove(&self, id: &[i64])-> Result<(), DatabaseError> {
        todo!()
    }
}