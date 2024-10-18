use std::rc::Rc;

use direct_access::DbConnectionTrait;
use direct_access::{root::RootRepositoryTrait, RepositoryFactoryTrait};
use crate::root::root_database_access::RootDatabaseAccess;
use crate::root::root_repository::RootRepository;

pub struct RepositoryFactory {
}

impl RepositoryFactory {
    pub fn new() -> RepositoryFactory {
        RepositoryFactory {
            
        }
    }
}

impl RepositoryFactoryTrait for RepositoryFactory {
    fn get_root_repository(&self, db_connection: Rc<dyn DbConnectionTrait>) -> Box<dyn RootRepositoryTrait> {
        let database = Box::new(RootDatabaseAccess::new(db_connection));
        Box::new(RootRepository::new(database))
    }
}