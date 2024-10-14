from abc import ABC, abstractmethod

from qleany.common.entities.field import Field

class IFieldDbTableGroup(ABC):

    @abstractmethod
    def get(self, ids) -> list[Field]:
        pass

    @abstractmethod
    def get_all(self) -> list[Field]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(self, entities) -> list[Field]:
        pass

    @abstractmethod
    def update(self, entities) -> list[Field]:
        pass

    @abstractmethod
    def remove(self, ids) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass