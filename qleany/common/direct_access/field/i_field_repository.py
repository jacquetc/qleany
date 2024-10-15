from abc import ABC, abstractmethod
from qleany.common.entities.field import Field
from typing import Sequence

from qleany.common.entities.entity_enums import EntityEnum


class IFieldRepository(ABC):
    @abstractmethod
    def get(self, ids: Sequence[int]) -> Sequence[Field]:
        pass

    @abstractmethod
    def get_all(self) -> Sequence[Field]:
        pass

    @abstractmethod
    def get_all_ids(self) -> Sequence[int]:
        pass

    @abstractmethod
    def create(
        self, entities: Sequence[Field]
    ) -> Sequence[Field]:
        pass

    @abstractmethod
    def update(
        self, entities: Sequence[Field]
    ) -> Sequence[Field]:
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
