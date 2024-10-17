from abc import ABC, abstractmethod

from qleany.common.direct_access.root.i_root_repository import IRootRepository


class IRootUow(ABC):
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
