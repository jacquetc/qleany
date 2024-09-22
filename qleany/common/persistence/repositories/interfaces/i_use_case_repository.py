from abc import ABC, abstractmethod
from qleany.common.entities.use_case import UseCase


class IUseCaseRepository(ABC):
    @abstractmethod
    def get(self, ids: list[int]) -> list[UseCase]:
        pass

    @abstractmethod
    def get_all(self) -> list[UseCase]:
        pass

    @abstractmethod
    def get_all_ids(self) -> list[int]:
        pass

    @abstractmethod
    def create(self, usecase_fields: list[UseCase]) -> list[UseCase]:
        pass

    @abstractmethod
    def update(self, usecase_fields: list[UseCase]) -> list[UseCase]:
        pass

    @abstractmethod
    def remove(self, ids: list[int]) -> list[int]:
        pass

    @abstractmethod
    def clear(self):
        pass
