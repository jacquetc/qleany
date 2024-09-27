import sqlite3
from abc import ABC, abstractmethod


class IDbConnection(ABC):

    @abstractmethod
    def connection(self) -> sqlite3.Connection:
        pass

    @abstractmethod
    def commit(self):
        pass
