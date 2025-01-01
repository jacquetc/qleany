
use common_entities::root::Root;
use crate::database::DatabaseAccessTrait;
use crate::direct_access::{root::RootRepositoryTrait, RepositoryError, RepositoryTrait};


pub(crate) trait RootDatabaseAccessTrait : DatabaseAccessTrait<Root> {}


pub(crate) struct RootRepository {
    database: Box<dyn RootDatabaseAccessTrait>,
}

impl RootRepository {
    pub fn new(database: Box<dyn RootDatabaseAccessTrait>) -> RootRepository {
        RootRepository {
            database,
        }
    }
}

impl RootRepositoryTrait for RootRepository {}

impl RepositoryTrait<Root> for RootRepository {
    fn get(&self, ids: &[i64]) -> Result<Vec<Root>, RepositoryError> {
        self.database.get(ids).map_err(|e| e.into())
    }
    fn update(&self, entities: &[Root]) -> Result<Vec<Root>, RepositoryError> {
        self.database.update(entities).map_err(|e| e.into())
    }
    fn remove(&self, ids: &[i64]) -> Result<(), RepositoryError> {
        self.database.remove(ids).map_err(|e| e.into())
    }
    fn create(&self, entities: &[Root]) -> Result<Vec<Root>, RepositoryError> {
        self.database.create(entities).map_err(|e| e.into())
    }

}

