from abc import ABC, abstractmethod
from qleany.common.entities.entity import Entity
from functools import lru_cache
from qleany.common.direct_access.common.repository.repository_observer import (
    RepositoryObserver,
    RepositorySubject,
)


class IEntityRepository(RepositoryObserver, RepositorySubject, ABC):
    @lru_cache(maxsize=None)
    @abstractmethod
    def get(self, ids: list[int]) -> list[Entity]:
        pass

    @abstractmethod
    def get_all(self) -> list[Entity]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(
        self, entities: list[Entity]
    ) -> list[Entity]:
        pass

    @abstractmethod
    def update(
        self, entities: list[Entity]
    ) -> list[Entity]:
        pass

    @abstractmethod
    def remove(self, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
