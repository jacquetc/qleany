from abc import ABC, abstractmethod
import sqlite3


class IDbConnection(ABC):

    @abstractmethod
    def connection(self) -> sqlite3.Connection:
        pass

    @abstractmethod
    def commit(self):
        pass
