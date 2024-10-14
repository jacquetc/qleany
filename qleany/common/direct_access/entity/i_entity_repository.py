from abc import ABC, abstractmethod
from qleany.common.entities.entity import Entity
from functools import lru_cache
from typing import Sequence, List

from qleany.common.entities.entity_enums import EntityEnum


class IEntityRepository(ABC):
    @abstractmethod
    def get(self, ids: Sequence[int]) -> List[Entity]:
        pass

    @abstractmethod
    def get_all(self) -> List[Entity]:
        pass

    @abstractmethod
    def get_all_ids(self) -> List[int]:
        pass

    @abstractmethod
    def create(
        self, entities: Sequence[Entity]
    ) -> List[Entity]:
        pass

    @abstractmethod
    def update(
        self, entities: Sequence[Entity]
    ) -> List[Entity]:
        pass

    @abstractmethod
    def remove(self, ids: Sequence[int]) -> List[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
