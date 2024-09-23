from abc import ABC, abstractmethod
import sqlite3


class IDbConnection(ABC):

    @abstractmethod
    def connection(self) -> sqlite3.connection:
        pass

    @abstractmethod
    def commit(self):
        pass
