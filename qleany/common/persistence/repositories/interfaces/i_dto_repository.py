from abc import ABC, abstractmethod
from qleany.common.entities.dto import Dto


class IDtoRepository(ABC):
    @abstractmethod
    def get(self, db_connection: IDbConnection, ids: list[int]) -> list[Dto]:
        pass

    @abstractmethod
    def get_all(self, db_connection: IDbConnection) -> list[Dto]:
        pass

    @abstractmethod
    def get_all_ids(self, db_connection: IDbConnection) -> list[int]:
        pass

    @abstractmethod
    def create(self, db_connection: IDbConnection, entities: list[Dto]) -> list[Dto]:
        pass

    @abstractmethod
    def update(self, db_connection: IDbConnection, entities: list[Dto]) -> list[Dto]:
        pass

    @abstractmethod
    def remove(self, db_connection: IDbConnection, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self, db_connection: IDbConnection):
        pass
