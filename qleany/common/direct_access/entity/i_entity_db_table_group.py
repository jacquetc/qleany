from abc import ABC, abstractmethod

from qleany.common.entities.entity import Entity

class IEntityDbTableGroup(ABC):

    @abstractmethod
    def get(self, ids) -> list[Entity]:
        pass

    @abstractmethod
    def get_all(self) -> list[Entity]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(self, entities) -> list[Entity]:
        pass

    @abstractmethod
    def update(self, entities) -> list[Entity]:
        pass

    @abstractmethod
    def remove(self, ids) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass

    @abstractmethod
    def get_left_ids(self, left_entity, field_name, right_ids) -> list[int]:
        pass

    @abstractmethod
    def get_right_ids(self, left_entity, field_name, left_ids) -> list[int]:
        pass