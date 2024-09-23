from abc import ABC, abstractmethod
import sqlite3
from qleany.common.persistence.database.interfaces.i_db_connection import IDbConnection


class IDbContext(ABC):

    @abstractmethod
    def get_connection(self) -> IDbConnection:
        pass
