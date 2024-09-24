from abc import ABC, abstractmethod
from qleany.common.entities.entity import Entity
from functools import lru_cache
from qleany.common.persistence.database.interfaces.i_db_connection import IDbConnection


class IEntityRepository(ABC):
    @lru_cache(maxsize=None)
    @abstractmethod
    def get(self, db_connection: IDbConnection, ids: list[int]) -> list[Entity]:
        pass

    @abstractmethod
    def get_all(self, db_connection: IDbConnection) -> list[Entity]:
        pass

    @abstractmethod
    def get_all_ids(self, db_connection: IDbConnection) -> list[int]:
        pass

    @abstractmethod
    def create(
        self, db_connection: IDbConnection, entities: list[Entity]
    ) -> list[Entity]:
        pass

    @abstractmethod
    def update(
        self, db_connection: IDbConnection, entities: list[Entity]
    ) -> list[Entity]:
        pass

    @abstractmethod
    def remove(self, db_connection: IDbConnection, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self, db_connection: IDbConnection):
        pass
