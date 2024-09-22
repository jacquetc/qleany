from abc import ABC, abstractmethod
from qleany.common.entities.dto import Dto


class IDtoRepository(ABC):
    @abstractmethod
    def get(self, ids: list[int]) -> list[Dto]:
        pass

    @abstractmethod
    def get_all(self) -> list[Dto]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(self, dto_fields: list[Dto]) -> list[Dto]:
        pass

    @abstractmethod
    def update(self, dto_fields: list[Dto]) -> list[Dto]:
        pass

    @abstractmethod
    def remove(self, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
