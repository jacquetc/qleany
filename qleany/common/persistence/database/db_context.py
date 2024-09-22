import sqlite3
import tempfile
import threading
import os
from typing import Dict, List

class DbContext:
    def __init__(self):
        self.mutex = threading.Lock()
        self.file_name = None
        self.database_name = None
        self._creation_sql_dict: Dict[str, List[str]] = {}

        try:
            # Initialize the internal database
            temp_file = tempfile.NamedTemporaryFile(delete=False)
            self.file_name = temp_file.name
            temp_file.close()
            self.database_name = self.create_empty_database()
        except Exception as e:
            raise RuntimeError(f"Error initializing database: {str(e)}")

    def __del__(self):
        # Destructor logic to remove the database file and close all connections
        if self.database_name:
            try:
                os.remove(self.database_name)
            except OSError as e:
                print(f"Error removing database file: {e}")

    def get_connection(self) -> sqlite3.Connection:
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
                "PRAGMA foreign_keys=ON"
            ]
            cursor = conn.cursor()
            for pragma in pragmas:
                cursor.execute(pragma)
            return conn

    def append_creation_sql(self, type: str, sql: str):
        with self.mutex:
            if type not in self._creation_sql_dict:
                self._creation_sql_dict[type] = []
            self._creation_sql_dict[type].append(sql)

    def create_empty_database(self):
        try:
            with self.get_connection() as conn:
                cursor = conn.cursor()
                
                # Create the entity tables in the database
                entity_tables = self._creation_sql_dict.get("entity_table")
                
                for table in entity_tables:
                    cursor.execute(table)
                    
                # Create the junction tables in the database
                junction_tables = self._creation_sql_dict.get("junction_table")

                for table in junction_tables:
                    cursor.execute(table)

                # Execute additional PRAGMA statements for optimization
                optimization_pragmas = [
                    "PRAGMA case_sensitive_like=true",
                    "PRAGMA journal_mode=MEMORY",
                    "PRAGMA temp_store=MEMORY",
                    "PRAGMA locking_mode=NORMAL",
                    "PRAGMA synchronous=OFF",
                    "PRAGMA recursive_triggers=ON",
                    "PRAGMA foreign_keys=ON"
                ]
                
                for pragma in optimization_pragmas:
                    cursor.execute(pragma)

                conn.commit()

        except Exception as e:
            raise RuntimeError(f"Error creating database: {str(e)}")


