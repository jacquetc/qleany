use crate::repository_factory::RepositoryFactory;
use common_persistence::database::sqlite_db_context::SqliteDbContext;

pub fn register() -> RepositoryFactory {
    let db_context = Box::new(SqliteDbContext::new());
    let factory = RepositoryFactory::new();

    factory
}
