from abc import ABC, abstractmethod
from typing import Sequence, Any
from qleany.common.entities.entity_enums import RelationshipInfo
from qleany.common.entities.i_entity import IEntity

class IDbTableGroup(ABC):

    @abstractmethod
    def get(self, ids: Sequence[int]) -> Sequence[IEntity]:
        pass

    @abstractmethod
    def get_all(self) -> Sequence[IEntity]:
        pass

    @abstractmethod
    def get_all_ids(self) -> Sequence[int]:
        pass

    @abstractmethod
    def create(self, entities: Sequence[IEntity]) -> Sequence[IEntity]:
        pass

    @abstractmethod
    def update(self, entities: Sequence[IEntity]) -> Sequence[IEntity]:
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

    @abstractmethod
    def get_left_ids(self, relationship: RelationshipInfo, right_id: int) -> Sequence[int]:
        pass

    @abstractmethod
    def get_right_ids(self, relationship: RelationshipInfo, left_id: int) -> Sequence[int]:
        pass