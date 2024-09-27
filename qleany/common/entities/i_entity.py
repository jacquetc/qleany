from abc import ABC, abstractmethod

from qleany.common.entities.entity_enums import EntitySchema


class IEntity(ABC):

    @abstractmethod
    @classmethod
    def _schema(cls) -> EntitySchema:
        pass
