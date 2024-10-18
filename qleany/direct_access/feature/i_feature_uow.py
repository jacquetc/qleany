from abc import ABC, abstractmethod

from qleany.common.direct_access.feature.i_feature_repository import IFeatureRepository
from qleany.common.direct_access.root.i_root_repository import IRootRepository


class IFeatureUow(ABC):
    @property
    @abstractmethod
    def feature_repository(self) -> IFeatureRepository:
        pass

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
