from abc import ABC, abstractmethod

from qleany.common.direct_access.entity.i_entity_repository import IEntityRepository
from qleany.common.direct_access.root.i_root_repository import IRootRepository

class IEntityUow(ABC):
    @property
    @abstractmethod
    def entity_repository(self) -> IEntityRepository:
        pass
    
    @property
    @abstractmethod
    def root_repository(self) -> IRootRepository:
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
