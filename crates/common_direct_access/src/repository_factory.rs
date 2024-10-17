use common_persistence::database::DbContextTrait;
use direct_access::{root::RootRepositoryTrait, RepositoryFactoryTrait};
use crate::root::root_database_access::RootDatabaseAccess;
use crate::root::root_repository::RootRepository;

pub struct RepositoryFactory {
    pub db_context: Box<dyn DbContextTrait>,
}

impl RepositoryFactory {
    pub fn new(db_context: Box<dyn DbContextTrait> ) -> RepositoryFactory {
        RepositoryFactory {
            db_context,
        }
    }
}

impl RepositoryFactoryTrait for RepositoryFactory {
    fn get_root_repository(&self) -> Box<dyn RootRepositoryTrait> {
        let database = Box::new(RootDatabaseAccess::new(self.db_context));
        Box::new(RootRepository::new(database))
    }
}