use crate::direct_access::repository_factory::RepositoryFactory;
use crate::database::sqlite_db_context::SqliteDbContext;

pub fn register() -> RepositoryFactory {
    let db_context = Box::new(SqliteDbContext::new());
    let factory = RepositoryFactory::new();

    factory
}
