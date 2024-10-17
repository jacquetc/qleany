from typing import Optional

from qleany_.common.direct_access.common.database.interfaces.i_db_connection import (
    IDbConnection,
)
from qleany_.common.direct_access.common.database.interfaces.i_db_context import (
    IDbContext,
)
from qleany_.common.direct_access.common.repository.repository_factory import (
    IRepositoryFactory,
)
from qleany_.common.direct_access.entity.i_entity_repository import IEntityRepository
from qleany_.common.direct_access.field.i_field_repository import IFieldRepository
from qleany_.direct_access.field.i_field_uow import IFieldUow


class FieldUow(IFieldUow):
    _context: IDbContext
    _connection: Optional[IDbConnection]
    _repository_factory: IRepositoryFactory

    def __init__(self, db_context: IDbContext, repository_factory: IRepositoryFactory):
        self._context = db_context
        self._repository_factory = repository_factory

    @property
    def field_repository(self) -> IFieldRepository:
        if self._connection is None:
            raise ValueError("Connection is not established.")
        return self._repository_factory.create("FieldRepository", self._connection)

    # owner:
    @property
    def entity_repository(self) -> IEntityRepository:
        if self._connection is None:
            raise ValueError("Connection is not established.")
        return self._repository_factory.create("EntityRepository", self._connection)

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
