from qleany.common.direct_access.field.i_field_db_table_group import IFieldDbTableGroup
from qleany.common.direct_access.field.i_field_repository import (
    IFieldRepository,
)
from qleany.common.entities.entity_enums import EntityEnum
from qleany.common.entities.field import Field
from functools import lru_cache
import logging


class FieldRepository(IFieldRepository, RepositorySubject):

    def __init__(self ,database: IFieldDbTableGroup):
        super().__init__()
        self._database = database
        self._cache = {}

    @lru_cache(maxsize=None)
    def get(self, ids: list[int]) -> list[Field]:
        cached_entities = [self._cache[id] for id in ids if id in self._cache]
        missing_ids = [id for id in ids if id not in self._cache]
        if missing_ids:
            db_entities = self._database.get(missing_ids)
            for entity in db_entities:
                self._cache[entity.id_] = entity
            cached_entities.extend(db_entities)
        return cached_entities

    def get_all(self) -> list[Field]:
        if not self._cache:
            db_entities = self._database.get_all()
            for entity in db_entities:
                self._cache[entity.id_] = entity
        return list(self._cache.values())

    def get_all_ids(self) -> list[int]:
        if not self._cache:
            self.get_all()
        return list(self._cache.keys())

    def create(
        self, entities: list[Field]
    ) -> list[Field]:
        created_entities = self._database.create(entities)
        for entity in created_entities:
            self._cache[entity.id_] = entity
        self.get.cache_clear()

        self._notify_created([entity.id_ for entity in created_entities])

        logging.info(f"Created {created_entities}")

        return created_entities

    def update(
        self, entities: list[Field]
    ) -> list[Field]:
        updated_entities = self._database.update(entities)
        for entity in updated_entities:
            if entity.id_ in self._cache:
                self._cache[entity.id_] = entity
        self.get.cache_clear()

        self._notify_updated([entity.id_ for entity in updated_entities])

        logging.info(f"Updated {updated_entities}")

        return updated_entities

    def remove(self, ids: list[int]) -> list[int]:

        # remove from database and cache
        self._database.remove(ids)
        for id in ids:
            if id in self._cache:
                del self._cache[id]
        self.get.cache_clear()

        # signals all repos depending of this repo
        for relationship in Field._schema().relationships:
            if relationship.relationship_direction == RelationshipDirection.Backward:
                left_ids = self._database.get_left_ids(
                    relationship.left_entity_name,
                    relationship.field_name,
                    ids,
                )
                self._notify_related_ids_to_be_cleared_from_cache(
                    relationship.left_entity, left_ids
                )

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
        self,
        left_entity: str,
        field_name: str,
        left_entity_ids: list[int],
    ):
        right_ids = self._database.get_right_ids(
            left_entity, field_name, left_entity_ids
        )
        self.remove(right_ids)
        logging.info(f"Cascade remove {right_ids} from {left_entity} {field_name}")
