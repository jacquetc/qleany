use crate::direct_access::DbConnectionTrait;
use tempfile::NamedTempFile;
use crate::database::DbContextTrait;
use super::{sqlite_db_connection::SqliteDbConnection, DatabaseError};

#[derive(Debug)]    
pub struct SqliteDbContext {
    temp_database: NamedTempFile,
}

impl SqliteDbContext {
    pub fn new() -> SqliteDbContext {
        SqliteDbContext {
            temp_database: NamedTempFile::new().unwrap(),
        }
    }
}
impl DbContextTrait for SqliteDbContext {
    fn create_connection(&self) -> Result<impl DbConnectionTrait, DatabaseError> {

        Ok(SqliteDbConnection::new(self.temp_database.path()))
    }
}
