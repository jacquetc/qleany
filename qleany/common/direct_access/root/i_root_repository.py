from abc import ABC, abstractmethod
from qleany.common.entities.root import Root
from typing import Sequence

from qleany.common.entities.entity_enums import EntityEnum


class IRootRepository(ABC):
    @abstractmethod
    def get(self, ids: Sequence[int]) -> Sequence[Root]:
        pass

    @abstractmethod
    def get_all(self) -> Sequence[Root]:
        pass

    @abstractmethod
    def get_all_ids(self) -> Sequence[int]:
        pass

    @abstractmethod
    def create(
        self, entities: Sequence[Root]
    ) -> Sequence[Root]:
        pass

    @abstractmethod
    def update(
        self, entities: Sequence[Root]
    ) -> Sequence[Root]:
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