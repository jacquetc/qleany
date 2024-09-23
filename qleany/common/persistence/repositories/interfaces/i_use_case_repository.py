from abc import ABC, abstractmethod
from qleany.common.entities.use_case import UseCase


class IUseCaseRepository(ABC):
    @abstractmethod
    def get(self, db_connection: IDbConnection, ids: list[int]) -> list[UseCase]:
        pass

    @abstractmethod
    def get_all(self, db_connection: IDbConnection) -> list[UseCase]:
        pass

    @abstractmethod
    def get_all_ids(self, db_connection: IDbConnection) -> list[int]:
        pass

    @abstractmethod
    def create(
        self, db_connection: IDbConnection, entities: list[UseCase]
    ) -> list[UseCase]:
        pass

    @abstractmethod
    def update(
        self, db_connection: IDbConnection, entities: list[UseCase]
    ) -> list[UseCase]:
        pass

    @abstractmethod
    def remove(self, db_connection: IDbConnection, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self, db_connection: IDbConnection):
        pass
