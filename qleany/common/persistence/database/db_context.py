import os
import tempfile
import threading

from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

from qleany.common.persistence.database.db_connection import DbConnection
from qleany.common.persistence.database.interfaces.i_db_connection import (
    IDbConnection,
)
from qleany.common.persistence.database.models import Base


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

            self.engine = create_engine(f'sqlite:///{self.file_name}')
            Base.metadata.create_all(self.engine)
            self.Session = sessionmaker(bind=self.engine)
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
            session = self.Session()
            return DbConnection(session)
