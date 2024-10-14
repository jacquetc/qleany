from qleany.common.direct_access.common.database.interfaces.i_db_context import IDbContext
from abc import ABC, abstractmethod

class IUnitOfWork(ABC):
    

    @abstractmethod
    def commit(self):
        pass

    @abstractmethod
    def rollback(self):
        pass

    @abstractmethod
    def __enter__(self) -> 'IUnitOfWork':
        pass

    @abstractmethod
    def __exit__(self, exc_type, exc_value, traceback):
        pass

class UnitOfWork(IUnitOfWork):
    

    def __init__(self, db_context: IDbContext):
        self._context = db_context

    def commit(self):
        self._connection.commit()

    def rollback(self):
        self._connection.rollback()

    def __enter__(self):
        self._connection = self._context.get_connection()
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        if exc_type is None:
            self.commit()
        else:
            self.rollback()
        self._connection.close()