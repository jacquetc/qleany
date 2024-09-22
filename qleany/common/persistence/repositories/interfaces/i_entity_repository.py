from abc import ABC, abstractmethod
from qleany.common.entities.entity import Entity


class IEntityRepository(ABC):
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
    def create(self, entity_fields: list[Entity]) -> list[Entity]:
        pass

    @abstractmethod
    def update(self, entity_fields: list[Entity]) -> list[Entity]:
        pass

    @abstractmethod
    def remove(self, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
