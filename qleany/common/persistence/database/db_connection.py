import sqlite3
from qleany.common.persistence.database.interfaces.i_db_connection import IDbConnection



class DbConnection(IDbConnection):
    def __init__(self, sqlite_connection: sqlite3.Connection):
        self._sqlite_connection = sqlite_connection

    def connnection(self) -> sqlite3.Connection:
        return self._sqlite_connection

    def commit(self):
        self._sqlite_connection.commit()
