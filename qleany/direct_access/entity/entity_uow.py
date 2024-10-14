import sqlite3
from qleany.common.direct_access.common.repository.repository_factory import IRepositoryFactory
from qleany.common.direct_access.common.repository.unit_of_work import UnitOfWork
from qleany.common.direct_access.common.repository.repository_factory import RepositoryFactory
from qleany.common.direct_access.entity.entity_repository import EntityRepository
from qleany.common.direct_access.entity.i_entity_repository import IEntityRepository
from qleany.direct_access.entity.i_entity_uow import IEntityUow
from qleany.common.direct_access.common.database.interfaces.i_db_context import IDbContext


class EntityUnitOfWork(IEntityUow, UnitOfWork):
    def __init__(self, db_context: IDbContext, repository_factory: IRepositoryFactory):
        super().__init__(db_context)
        self._repository_factory = repository_factory

    @property
    def entity_repository(self) -> IEntityRepository:
        return self._repository_factory.create("EntityRepository", super()._connection)


