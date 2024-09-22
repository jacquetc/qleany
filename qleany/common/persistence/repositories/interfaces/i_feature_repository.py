from abc import ABC, abstractmethod
from qleany.common.entities.feature import Feature


class IFeatureRepository(ABC):
    @abstractmethod
    def get(self, ids: list[int]) -> list[Feature]:
        pass

    @abstractmethod
    def get_all(self) -> list[Feature]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(self, feature_fields: list[Feature]) -> list[Feature]:
        pass

    @abstractmethod
    def update(self, feature_fields: list[Feature]) -> list[Feature]:
        pass

    @abstractmethod
    def remove(self, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
