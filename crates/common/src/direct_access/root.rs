pub(crate) mod root_database_access;
pub(crate) mod root_repository;


use common_entities::root::Root;
use crate::direct_access::RepositoryTrait;

pub trait RootRepositoryTrait : RepositoryTrait<Root> {}