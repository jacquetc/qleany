from abc import ABC, abstractmethod

from qleany.common.direct_access.field.i_field_repository import IFieldRepository
from qleany.common.direct_access.entity.i_entity_repository import IEntityRepository

class IFieldUow(ABC):
    @property
    @abstractmethod
    def field_repository(self) -> IFieldRepository:
        pass
    
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
