from abc import ABC, abstractmethod
from qleany.common.entities.feature import Feature


class IFeatureRepository(ABC):
    @abstractmethod
    def get(self, db_connection: IDbConnection, ids: list[int]) -> list[Feature]:
        pass

    @abstractmethod
    def get_all(self, db_connection: IDbConnection) -> list[Feature]:
        pass

    @abstractmethod
    def get_all_ids(self, db_connection: IDbConnection) -> list[int]:
        pass

    @abstractmethod
    def create(
        self, db_connection: IDbConnection, entities: list[Feature]
    ) -> list[Feature]:
        pass

    @abstractmethod
    def update(
        self, db_connection: IDbConnection, entities: list[Feature]
    ) -> list[Feature]:
        pass

    @abstractmethod
    def remove(self, db_connection: IDbConnection, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self, db_connection: IDbConnection):
        pass
