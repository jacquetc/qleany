from abc import ABC, abstractmethod
from qleany.common.entities.dto_field import DtoField

class IDtoFieldRepository(ABC):
    @abstractmethod
    def get(self, ids: list[int]) -> list[DtoField]:
        pass

    @abstractmethod
    def get_all(self) -> list[DtoField]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(self, dto_fields: list[DtoField]) -> list[DtoField]:
        pass

    @abstractmethod
    def update(self, dto_fields: list[DtoField]) -> list[DtoField]:
        pass

    @abstractmethod
    def delete(self, ids: list[int]) -> list[int]:
        pass