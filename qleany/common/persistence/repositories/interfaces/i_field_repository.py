from abc import ABC, abstractmethod
from qleany.common.entities.field import Field


class IFieldRepository(ABC):
    @abstractmethod
    def get(self, ids: list[int]) -> list[Field]:
        pass

    @abstractmethod
    def get_all(self) -> list[Field]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(self, field_fields: list[Field]) -> list[Field]:
        pass

    @abstractmethod
    def update(self, field_fields: list[Field]) -> list[Field]:
        pass

    @abstractmethod
    def remove(self, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
