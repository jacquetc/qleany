use std::rc::Rc;

use crate::direct_access::DbConnectionTrait;
use crate::direct_access::{root::RootRepositoryTrait, RepositoryFactoryTrait};
use crate::direct_access::root::root_database_access::RootDatabaseAccess;
use crate::direct_access::root::root_repository::RootRepository;

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
        let database = Box::new(RootDatabaseAccess::new(db_connection.clone()));
        Box::new(RootRepository::new(database))
    }
}