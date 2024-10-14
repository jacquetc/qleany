from abc import ABC, abstractmethod
from qleany.common.entities.field import Field
from functools import lru_cache
from qleany.common.direct_access.common.repository.repository_observer import (
    RepositorySubject,
)

class IFieldRepository(RepositorySubject, ABC):
    @lru_cache(maxsize=None)
    @abstractmethod
    def get(self, ids: list[int]) -> list[Field]:
        pass

    @abstractmethod
    def get_all(self) -> list[Field]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(
        self, entities: list[Field]
    ) -> list[Field]:
        pass

    @abstractmethod
    def update(
        self, entities: list[Field]
    ) -> list[Field]:
        pass

    @abstractmethod
    def remove(self, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
