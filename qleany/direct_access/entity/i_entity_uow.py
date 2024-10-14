from abc import ABC, abstractmethod

from qleany.common.direct_access.common.repository.unit_of_work import IUnitOfWork
from qleany.common.direct_access.entity.i_entity_repository import IEntityRepository

class IEntityUow(IUnitOfWork, ABC):
    @property
    @abstractmethod
    def entity_repository(self) -> IEntityRepository:
        pass
