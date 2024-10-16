from abc import ABC, abstractmethod

from qleany.common.entities.entity_enums import EntitySchema


class IEntity(ABC):
    id_: int = 0

    @classmethod
    @abstractmethod
    def _schema(cls) -> EntitySchema:
        pass
