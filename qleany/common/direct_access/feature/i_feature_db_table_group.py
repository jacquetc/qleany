from abc import ABC, abstractmethod
from typing import Sequence
from qleany.common.entities.entity_enums import RelationshipInfo
from qleany.common.entities.feature import Feature

class IFeatureDbTableGroup(ABC):

    @abstractmethod
    def get(self, ids: Sequence[int]) -> Sequence[Feature]:
        pass

    @abstractmethod
    def get_all(self) -> Sequence[Feature]:
        pass

    @abstractmethod
    def get_all_ids(self) -> Sequence[int]:
        pass

    @abstractmethod
    def create(self, entities: Sequence[Feature]) -> Sequence[Feature]:
        pass

    @abstractmethod
    def update(self, entities: Sequence[Feature]) -> Sequence[Feature]:
        pass

    @abstractmethod
    def remove(self, ids: Sequence[int]) -> Sequence[int]:
        pass

    @abstractmethod
    def clear(self):
        pass

    @abstractmethod
    def get_left_ids(self, relationship: RelationshipInfo, right_id: int) -> Sequence[int]:
        pass

    @abstractmethod
    def get_right_ids(self, relationship: RelationshipInfo, left_id: int) -> Sequence[int]:
        pass