from qleany.common.persistence.repositories.field_repository import (
    FieldRepository,
)
from qleany.common.persistence.repositories.relationship_repository import (
    RelationshipRepository,
)

from qleany.common.persistence.repositories.interfaces.i_entity_repository import (
    IEntityRepository,
)
from qleany.common.entities.entity import Entity
from functools import lru_cache
from qleany.common.persistence.repositories.repository_observer import (
    RepositoryObserver,
    RepositorySubject,
)
import logging
from qleany.common.entities.entity import Entity


class EntityRepository(IEntityRepository, RepositoryObserver, RepositorySubject):

    def __init__(
        self,
        db_context,
        field_repository: FieldRepository,
        relationship_repository: RelationshipRepository,
    ):
        self._database = Database(db_context)
        self._field_repository = field_repository
        self._relationship_repository = relationship_repository

        self._field_repository.attach(self)
        self._relationship_repository.attach(self)

        self._cache = {}

    @lru_cache(maxsize=None)
    def get(self, ids: list[int]) -> list[Entity]:
        cached_entities = [self._cache[id] for id in ids if id in self._cache]
        missing_ids = [id for id in ids if id not in self._cache]
        if missing_ids:
            db_entities = self._database.get(missing_ids)
            for entity in db_entities:
                self._cache[entity.id_] = entity
            cached_entities.extend(db_entities)
        return cached_entities

    def get_all(self) -> list[Entity]:
        if not self._cache:
            db_entities = self._database.get_all()
            for entity in db_entities:
                self._cache[entity.id_] = entity
        return list(self._cache.values())

    def get_all_ids(self) -> list[int]:
        if not self._cache:
            self.get_all()
        return list(self._cache.keys())

    def create(self, entities: list[Entity]) -> list[Entity]:
        created_entities = self._database.create(entities)
        for entity in created_entities:
            self._cache[entity.id_] = entity
        self.get.cache_clear()

        self._notify_created([entity.id_ for entity in created_entities])

        logging.info(f"Created {created_entities}")

        return created_entities

    def update(self, entities: list[Entity]) -> list[Entity]:
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
        self._field_repository.cascade_remove("Entity", "fields", ids)
        self._relationship_repository.cascade_remove("Entity", "relationships", ids)

        removed_ids = self._database.remove(ids)
        for id in removed_ids:
            if id in self._cache:
                del self._cache[id]
        self.get.cache_clear()

        self._notify_removed(removed_ids)

        logging.info(f"Removed {removed_ids}")

        return removed_ids

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

    # observer methods

    def _on_related_ids_to_be_cleared_from_cache(
        self, subject, left_entity: type, left_ids: list[int]
    ):
        if subject not in [
            self._field_repository,
            self._relationship_repository,
        ]:
            return
        if left_entity != Entity:
            return
        self.get.cache_clear()
        [self._cache.pop(id, None) for id in left_ids]