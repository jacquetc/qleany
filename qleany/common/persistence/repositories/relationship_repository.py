from qleany.common.persistence.repositories.interfaces.i_relationship_repository import (
    IRelationshipRepository,
)
from qleany.common.entities.relationship import Relationship
from functools import lru_cache
import logging
from qleany.common.persistence.repositories.repository_observer import RepositorySubject


class RelationshipRepository(IRelationshipRepository, RepositorySubject):

    def __init__(self):
        self._database = Database(db_context)
        self._cache = {}

    @lru_cache(maxsize=None)
    def get(self, db_connection: IDbConnection, ids: list[int]) -> list[Relationship]:
        cached_entities = [self._cache[id] for id in ids if id in self._cache]
        missing_ids = [id for id in ids if id not in self._cache]
        if missing_ids:
            db_entities = self._database.get(db_connection, missing_ids)
            for entity in db_entities:
                self._cache[entity.id_] = entity
            cached_entities.extend(db_entities)
        return cached_entities

    def get_all(self, db_connection: IDbConnection) -> list[Relationship]:
        if not self._cache:
            db_entities = self._database.get_all()
            for entity in db_entities:
                self._cache[entity.id_] = entity
        return list(self._cache.values())

    def get_all_ids(self, db_connection: IDbConnection) -> list[int]:
        if not self._cache:
            self.get_all()
        return list(self._cache.keys())

    def create(
        self, db_connection: IDbConnection, entities: list[Relationship]
    ) -> list[Relationship]:
        created_entities = self._database.create(entities)
        for entity in created_entities:
            self._cache[entity.id_] = entity
        self.get.cache_clear()

        self._notify_created([entity.id_ for entity in created_entities])

        logging.info(f"Created {created_entities}")

        return created_entities

    def update(
        self, db_connection: IDbConnection, entities: list[Relationship]
    ) -> list[Relationship]:
        updated_entities = self._database.update(entities)
        for entity in updated_entities:
            if entity.id_ in self._cache:
                self._cache[entity.id_] = entity
        self.get.cache_clear()

        self._notify_updated([entity.id_ for entity in updated_entities])

        logging.info(f"Updated {updated_entities}")

        return updated_entities

    def remove(self, db_connection: IDbConnection, ids: list[int]) -> list[int]:

        # remove from database and cache
        self._database.remove(ids)
        for id in ids:
            if id in self._cache:
                del self._cache[id]
        self.get.cache_clear()

        self._notify_removed(ids)

        logging.info(f"Removed {ids}")

        return ids

    def clear(self, db_connection: IDbConnection):
        self._database.clear()
        self._cache.clear()
        self.get.cache_clear()

        self._notify_cleared()

        logging.info("Cache cleared")

    def cascade_remove(
        self,
        db_connection: IDbConnection,
        left_entity: str,
        relationship_name: str,
        left_entity_ids: list[int],
    ):
        right_ids = self._database.get_right_ids(
            db_connection, left_entity, relationship_name, left_entity_ids
        )
        self.remove(right_ids)
        logging.info(
            f"Cascade remove {right_ids} from {left_entity} {relationship_name}"
        )
