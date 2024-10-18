from abc import ABC, abstractmethod
from typing import Sequence

from qleany.common.entities.field import Field


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
        self, entities: Sequence[Field], owner_id: int, position: int
    ) -> Sequence[Field]:
        pass

    @abstractmethod
    def update(self, entities: Sequence[Field]) -> Sequence[Field]:
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
