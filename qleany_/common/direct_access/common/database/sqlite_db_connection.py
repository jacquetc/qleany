import sqlite3

from qleany_.common.direct_access.common.database.interfaces.i_db_connection import (
    IDbConnection,
)


class SqliteDbConnection(IDbConnection):
    def __init__(self, sqlite_connection: sqlite3.Connection):
        self._sqlite_connection = sqlite_connection

    def connection(self) -> sqlite3.Connection:
        return self._sqlite_connection

    def commit(self):
        self._sqlite_connection.commit()

    def rollback(self):
        self._sqlite_connection.rollback()

    def close(self):
        self._sqlite_connection.close()
