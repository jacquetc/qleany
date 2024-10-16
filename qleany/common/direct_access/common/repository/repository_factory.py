from abc import ABC, abstractmethod
from typing import Type, Dict, Sequence, Any, Tuple

from qleany.common.direct_access.common.database.interfaces.i_db_connection import IDbConnection
from qleany.common.direct_access.common.repository.repository_messenger import IMessenger, Messenger


class IRepositoryFactory(ABC):

    @abstractmethod
    def register(self, repo_type: Type, table_group: Type):
        pass

    @abstractmethod
    def create(self, repo_types: str, db_connection: IDbConnection) -> Any:
        pass

    @abstractmethod
    def create_several(self, repo_types: Tuple[str], db_connection: IDbConnection) -> Tuple[Any]:
        pass
    
    


class RepositoryFactory(IRepositoryFactory):
    _repositories: Dict[str, Type] = {}
    _table_group_cache: Dict[str, Type] = {}
    _messenger: IMessenger
    
    def __init__(self, messenger: IMessenger):
        self._messenger = messenger

    def register(self, repo_type: Type, table_group: type):
        repo_name = repo_type.__name__
        self._repositories[repo_name] = repo_type
        self._table_group_cache[repo_name] = table_group

    def create(self, repo_types: str, db_connection: IDbConnection) -> Any:
        return self._repositories[repo_types](self._table_group_cache[repo_types](db_connection), db_connection, self, self._messenger)
    
    def create_several(self, repo_types: Tuple[str], db_connection: IDbConnection) -> Tuple[Any]:
        return tuple([self._repositories[repo_type](self._table_group_cache[repo_type](db_connection), db_connection, self, self._messenger) for repo_type in repo_types])
    