from abc import ABC, abstractmethod
from qleany.common.entities.root import Root


class IRootRepository(ABC):
    @abstractmethod
    def get(self, ids: list[int]) -> list[Root]:
        pass

    @abstractmethod
    def get_all(self) -> list[Root]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(self, root_fields: list[Root]) -> list[Root]:
        pass

    @abstractmethod
    def update(self, root_fields: list[Root]) -> list[Root]:
        pass

    @abstractmethod
    def remove(self, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
