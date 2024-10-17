from abc import ABC, abstractmethod
from qleany_.common.direct_access.common.database.interfaces.i_db_connection import IDbConnection


class IDbContext(ABC):

    @abstractmethod
    def get_connection(self) -> IDbConnection:
        pass
