from abc import ABC, abstractmethod
from qleany.common.entities.feature import Feature
from typing import Sequence

class IFeatureRepository(ABC):
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
    def create(
        self, entities: Sequence[Feature], owner_id: int, position: int
    ) -> Sequence[Feature]:
        pass

    @abstractmethod
    def update(
        self, entities: Sequence[Feature]
    ) -> Sequence[Feature]:
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