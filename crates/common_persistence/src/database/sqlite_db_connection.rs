use direct_access::DbConnectionTrait;
use anyhow::Result;

pub struct SqliteDbConnection<'a> {
    connection: rusqlite::Connection,
    transaction: Option<rusqlite::Transaction<'a>>,
}

impl<'a> SqliteDbConnection<'a> {
    pub fn new(connection: rusqlite::Connection) -> SqliteDbConnection {
        SqliteDbConnection { connection }
    }
}

impl<'a> DbConnectionTrait for SqliteDbConnection<'a> {
    fn begin_transaction(&self) -> Result<()> {
        self.transaction = Some(self.connection.transaction()?);
    }

    fn commit(&self) -> Result<(), direct_access::RepositoryError> {
        todo!()
    }

    fn rollback(&self) -> Result<(), direct_access::RepositoryError> {
        todo!()
    }
}