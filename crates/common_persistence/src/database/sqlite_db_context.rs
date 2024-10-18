use tempfile::NamedTempFile;
use crate::database::DbContextTrait;
use rusqlite::Connection;

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
    fn get_connection(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        Connection::open(self.temp_database.path())
    }
}
