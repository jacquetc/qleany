from typing import Type, Dict

from abc import ABC, abstractmethod
from typing import Type, Dict

class IRepositoryProvider(ABC):
    @classmethod
    @abstractmethod
    def register(cls, repo_type: Type, repo_instance: object):
        pass

    @classmethod
    @abstractmethod
    def get(cls, repo_type: Type):
        pass


class RepositoryProvider(IRepositoryProvider):
    _repositories: Dict[Type, object] = {}

    @classmethod
    def register(cls, repo_type: Type, repo_instance: object):
        cls._repositories[repo_type] = repo_instance

    @classmethod
    def get(cls, repo_type: Type):
        return cls._repositories.get(repo_type)
