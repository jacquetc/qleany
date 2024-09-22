from abc import ABC, abstractmethod
from qleany.common.entities.global_ import Global


class IGlobalRepository(ABC):
    @abstractmethod
    def get(self, ids: list[int]) -> list[Global]:
        pass

    @abstractmethod
    def get_all(self) -> list[Global]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(self, global_fields: list[Global]) -> list[Global]:
        pass

    @abstractmethod
    def update(self, global_fields: list[Global]) -> list[Global]:
        pass

    @abstractmethod
    def remove(self, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
