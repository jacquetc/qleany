from qleany.common.entities.entity_enums import RelationshipDirection
from qleany.common.persistence.repositories.interfaces.i_dto_field_repository import (
    IDtoFieldRepository,
)
from qleany.common.entities.dto_field import DtoField
from functools import lru_cache
import logging
from qleany.common.persistence.repositories.repository_observer import RepositorySubject


class DtoFieldRepository(IDtoFieldRepository, RepositorySubject):

    def __init__(self, db_context):
        self._database = Database(db_context)
        self._cache = {}

    @lru_cache(maxsize=None)
    def get(self, ids: list[int]) -> list[DtoField]:
        cached_entities = [self._cache[id] for id in ids if id in self._cache]
        missing_ids = [id for id in ids if id not in self._cache]
        if missing_ids:
            db_entities = self._database.get(missing_ids)
            for entity in db_entities:
                self._cache[entity.id_] = entity
            cached_entities.extend(db_entities)
        return cached_entities

    def get_all(self) -> list[DtoField]:
        if not self._cache:
            db_entities = self._database.get_all()
            for entity in db_entities:
                self._cache[entity.id_] = entity
        return list(self._cache.values())

    def get_all_ids(self) -> list[int]:
        if not self._cache:
            self.get_all()
        return list(self._cache.keys())

    def create(self, entities: list[DtoField]) -> list[DtoField]:
        created_entities = self._database.create(entities)
        for entity in created_entities:
            self._cache[entity.id_] = entity
        self.get.cache_clear()

        self._notify_created([entity.id_ for entity in created_entities])

        logging.info(f"Created {created_entities}")

        return created_entities

    def update(self, entities: list[DtoField]) -> list[DtoField]:
        updated_entities = self._database.update(entities)
        for entity in updated_entities:
            if entity.id_ in self._cache:
                self._cache[entity.id_] = entity
        self.get.cache_clear()

        self._notify_updated([entity.id_ for entity in updated_entities])

        logging.info(f"Updated {updated_entities}")

        return updated_entities

    def remove(self, ids: list[int]) -> list[int]:
        # cascade remove for strong relationships
        self._feature_repository.cascade_remove("Root", "features", ids)
        
        # remove from database and cache
        self._database.remove(ids)
        for id in ids:
            if id in self._cache:
                del self._cache[id]
        self.get.cache_clear()

        self._notify_removed(ids)

        logging.info(f"Removed {ids}")

        return ids

    def clear(self):
        self._database.clear()
        self._cache.clear()
        self.get.cache_clear()

        self._notify_cleared()

        logging.info("Cache cleared")

    def cascade_remove(
        self, left_entity: str, field_name: str, left_entity_ids: list[int]
    ):
        right_ids = self._database.get_right_ids(
            left_entity, field_name, left_entity_ids
        )
        self.remove(right_ids)
        logging.info(f"Cascade remove {right_ids} from {left_entity} {field_name}")
