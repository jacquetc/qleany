use std::sync::Arc;

use redb::{Database, Error};

#[derive(Clone, Debug)]
pub struct DbContext {
    database: Arc<Database>,
}

impl DbContext {
    pub fn new() -> Result<Self, Error> {
        let db = DbContext::create_db_in_memory()?;
        Ok(DbContext {
        database: Arc::new(db),
        })
    }

    fn create_db_in_memory() -> Result<Database, Error> {
        let redb_builder = redb::Builder::new();
        let in_memory_backend = redb::backends::InMemoryBackend::new();
        let db = redb_builder.create_with_backend(in_memory_backend)?;
        Ok(db)
    }

    pub fn get_database(&self) -> &Database {
        &self.database
    }
}
