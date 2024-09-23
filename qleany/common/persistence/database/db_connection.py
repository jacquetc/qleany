import sqlite3


class DbConnection(IDbConnection):
    def __init__(self, sqlite_connection: sqlite3.connection):
        self.sqlite_connection = sqlite_connection

    def connnection() -> sqlite3.connection:
        return self.sqlite_connection

    def commit():
        self.sqlite_connection.commit()
