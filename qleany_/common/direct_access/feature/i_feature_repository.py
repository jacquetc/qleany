from abc import ABC, abstractmethod
from typing import Sequence

from qleany_.common.entities.feature import Feature


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
    def update(self, entities: Sequence[Feature]) -> Sequence[Feature]:
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
