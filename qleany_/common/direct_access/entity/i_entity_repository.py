from abc import ABC, abstractmethod
from typing import Sequence

from qleany_.common.entities.entity import Entity


class IEntityRepository(ABC):
    @abstractmethod
    def get(self, ids: Sequence[int]) -> Sequence[Entity]:
        pass

    @abstractmethod
    def get_all(self) -> Sequence[Entity]:
        pass

    @abstractmethod
    def get_all_ids(self) -> Sequence[int]:
        pass

    @abstractmethod
    def create(
        self, entities: Sequence[Entity], owner_id: int, position: int
    ) -> Sequence[Entity]:
        pass

    @abstractmethod
    def update(self, entities: Sequence[Entity]) -> Sequence[Entity]:
        pass

    @abstractmethod
    def remove(self, ids: Sequence[int]) -> Sequence[int]:
        pass

    @abstractmethod
    def clear(self):
        pass

    @abstractmethod
    def exists(self, id_: int) -> bool:
        pass
