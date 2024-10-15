from qleany.common.direct_access.common.database.interfaces.i_db_connection import IDbConnection
from qleany.common.direct_access.common.repository.repository_factory import IRepositoryFactory
from qleany.common.direct_access.common.repository.repository_factory import RepositoryFactory
from qleany.common.direct_access.entity.entity_repository import EntityRepository
from qleany.common.direct_access.entity.i_entity_repository import IEntityRepository
from qleany.common.direct_access.root.i_root_repository import IRootRepository
from qleany.direct_access.entity.i_entity_uow import IEntityUow
from qleany.common.direct_access.common.database.interfaces.i_db_context import IDbContext
from typing import Optional

class EntityUow(IEntityUow):
    _context: IDbContext
    _connection: Optional[IDbConnection]
    _repository_factory: IRepositoryFactory
    
    
    def __init__(self, db_context: IDbContext, repository_factory: IRepositoryFactory):
        self._context = db_context
        self._repository_factory = repository_factory

    @property
    def entity_repository(self) -> IEntityRepository:
        if self._connection is None:
            raise ValueError("Connection is not established.")
        return self._repository_factory.create("EntityRepository", self._connection)
    
    # owner:
    @property
    def root_repository(self) -> IRootRepository:
        if self._connection is None:
            raise ValueError("Connection is not established.")
        return self._repository_factory.create("RootRepository", self._connection)

    def commit(self):
        if self._connection:
            self._connection.commit()

    def rollback(self):
        if self._connection:
            self._connection.rollback()

    def __enter__(self):
        self._connection = self._context.get_connection()
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        if exc_type is None:
            self.commit()
        else:
            self.rollback()
        if self._connection:
            self._connection.close()
            self._connection = None