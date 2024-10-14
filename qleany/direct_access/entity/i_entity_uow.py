from abc import ABC, abstractmethod

from qleany.common.direct_access.entity.i_entity_repository import IEntityRepository

class IEntityUow(ABC):
    @property
    @abstractmethod
    def entity_repository(self) -> IEntityRepository:
        pass

    @abstractmethod
    def commit(self):
        pass

    @abstractmethod
    def rollback(self):
        pass

    @abstractmethod
    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        pass
