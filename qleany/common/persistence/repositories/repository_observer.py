from qleany.common.persistence.database.db_table_group import DbTableGroup
from qleany.common.persistence.database.interfaces.i_db_connection import IDbConnection
from abc import ABC
from qleany.common.entities.entity_enums import EntityEnum


class RepositoryObserver(ABC):
    def _on_created(self, subject, ids: list[int]):
        pass

    def _on_updated(self, subject, ids: list[int]):
        pass

    def _on_removed(self, subject, ids: list[int]):
        pass

    def _on_cleared(self, subject):
        pass

    def _on_related_ids_to_be_cleared_from_cache(
        self, subject, left_entity, field_name, left_ids: list[int]
    ):
        pass


class RepositorySubject(ABC):
    def __init__(self):
        super().__init__()
        self._observers: list[RepositoryObserver] = []

    def attach(self, observer: RepositoryObserver):
        self._observers.append(observer)

    def detach(self, observer: RepositoryObserver):
        self._observers.remove(observer)

    def _notify_created(self, ids: list[int]):
        for observer in self._observers:
            observer._on_created(self, ids)

    def _notify_updated(self, ids: list[int]):
        for observer in self._observers:
            observer._on_updated(self, ids)

    def _notify_removed(self, ids: list[int]):
        for observer in self._observers:
            observer._on_removed(self, ids)

    def _notify_cleared(self):
        for observer in self._observers:
            observer._on_cleared(self)

    def _notify_related_ids_to_be_cleared_from_cache(
        self, left_entity: EntityEnum, left_ids: list[int]
    ):
        for observer in self._observers:
            observer._on_related_ids_to_be_cleared_from_cache(
                self, left_entity, left_ids
            )
