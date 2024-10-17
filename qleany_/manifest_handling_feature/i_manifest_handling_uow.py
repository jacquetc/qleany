from abc import ABC, abstractmethod

from qleany_.common.direct_access.entity.i_entity_repository import IEntityRepository
from qleany_.common.direct_access.feature.i_feature_repository import IFeatureRepository
from qleany_.common.direct_access.field.i_field_repository import IFieldRepository
from qleany_.common.direct_access.root.i_root_repository import IRootRepository


class IManifestHandlingUow(ABC):
    @property
    @abstractmethod
    def root_repository(self) -> IRootRepository:
        pass

    @property
    @abstractmethod
    def feature_repository(self) -> IFeatureRepository:
        pass

    @property
    @abstractmethod
    def entity_repository(self) -> IEntityRepository:
        pass

    @property
    @abstractmethod
    def field_repository(self) -> IFieldRepository:
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
