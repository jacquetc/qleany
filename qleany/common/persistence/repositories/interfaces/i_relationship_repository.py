from abc import ABC, abstractmethod
from qleany.common.entities.relationship import Relationship


class IRelationshipRepository(ABC):
    @abstractmethod
    def get(self, ids: list[int]) -> list[Relationship]:
        pass

    @abstractmethod
    def get_all(self) -> list[Relationship]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(self, relationship_fields: list[Relationship]) -> list[Relationship]:
        pass

    @abstractmethod
    def update(self, relationship_fields: list[Relationship]) -> list[Relationship]:
        pass

    @abstractmethod
    def remove(self, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
