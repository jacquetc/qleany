import os
import sqlite3
import tempfile
import threading

from qleany.common.persistence.database.db_connection import DbConnection
from qleany.common.persistence.database.interfaces.i_db_connection import (
    IDbConnection,
)


class DbContext:
    def __init__(self):
        self.mutex = threading.Lock()
        self.file_name = ""
        self.database_name = None

        try:
            # Initialize the internal database
            temp_file = tempfile.NamedTemporaryFile(delete=False)
            self.file_name = temp_file.name
            temp_file.close()
        except Exception as e:
            raise RuntimeError(f"Error initializing database: {str(e)}")

    def __del__(self):
        # Destructor logic to remove the database file and close all connections
        if self.database_name:
            try:
                os.remove(self.database_name)
            except OSError as e:
                print(f"Error removing database file: {e}")

    def get_connection(self) -> IDbConnection:
        with self.mutex:
            conn = sqlite3.connect(self.file_name)
            # Execute PRAGMA statements for the new connection
            pragmas = [
                "PRAGMA case_sensitive_like=true",
                "PRAGMA journal_mode=MEMORY",
                "PRAGMA temp_store=MEMORY",
                "PRAGMA locking_mode=NORMAL",
                "PRAGMA synchronous=OFF",
                "PRAGMA recursive_triggers=ON",
                "PRAGMA foreign_keys=ON",
            ]
            cursor = conn.cursor()
            for pragma in pragmas:
                cursor.execute(pragma)
            return DbConnection(conn)
