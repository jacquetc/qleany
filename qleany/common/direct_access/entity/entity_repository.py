from qleany.common.direct_access.common.database.interfaces.i_db_connection import IDbConnection
from qleany.common.direct_access.common.repository.repository_factory import IRepositoryFactory
from qleany.common.direct_access.entity.i_entity_db_table_group import IEntityDbTableGroup
from qleany.common.persistence.repositories.interfaces.i_field_repository import IFieldRepository
from qleany.common.persistence.repositories.relationship_repository import (
    RelationshipRepository,
)

from qleany.common.direct_access.entity.i_entity_repository import (
    IEntityRepository,
)
from qleany.common.entities.entity_enums import EntityEnum, RelationshipDirection
from functools import lru_cache
import logging
from qleany.common.entities.entity import Entity


class EntityRepository(IEntityRepository):

    def __init__(
        self,
        db_table_group: IEntityDbTableGroup,
        db_connection: IDbConnection,
        repository_factory: IRepositoryFactory
    ):
        super().__init__()
        self._db_table_group = db_table_group
        self._field_repository = repository_factory.create("FieldRepository", db_connection)
        # self._relationship_repository = repository_factory.create("RelatinonshipRepository", db_connection)

        self._field_repository.attach(self)
        # self._relationship_repository.attach(self)

        self._cache = {}

    @lru_cache(maxsize=None)
    def get(self, ids: list[int]) -> list[Entity]:
        cached_entities = [self._cache[id] for id in ids if id in self._cache]
        missing_ids = [id for id in ids if id not in self._cache]
        if missing_ids:
            db_entities = self._db_table_group.get(missing_ids)
            for entity in db_entities:
                self._cache[entity.id_] = entity
            cached_entities.extend(db_entities)
        return cached_entities

    def get_all(self) -> list[Entity]:
        if not self._cache:
            db_entities = self._db_table_group.get_all()
            for entity in db_entities:
                self._cache[entity.id_] = entity
        return list(self._cache.values())

    def get_all_ids(self) -> list[int]:
        if not self._cache:
            self.get_all()
        return list(self._cache.keys())

    def create(
        self, entities: list[Entity]
    ) -> list[Entity]:
        created_entities = self._db_table_group.create(entities)
        for entity in created_entities:
            self._cache[entity.id_] = entity
        self.get.cache_clear()

        self._notify_created([entity.id_ for entity in created_entities])

        logging.info(f"Created {created_entities}")

        return created_entities

    def update(
        self, entities: list[Entity]
    ) -> list[Entity]:
        updated_entities = self._db_table_group.update(entities)
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
        # self._relationship_repository.cascade_remove(
        #     db_connection, "Entity", "relationships", ids
        # )

        removed_ids = self._db_table_group.remove(ids)
        for id in removed_ids:
            if id in self._cache:
                del self._cache[id]
        self.get.cache_clear()

        # signals all repos depending of this repo
        for relationship in Entity._schema().relationships:
            if relationship.relationship_direction == RelationshipDirection.Backward:
                left_ids = self._db_table_group.get_left_ids(
                    relationship.left_entity_name,
                    relationship.field_name,
                    ids,
                )
                self._notify_related_ids_to_be_cleared_from_cache(
                    relationship.left_entity, left_ids
                )

        self._notify_removed(removed_ids)

        logging.info(f"Removed {removed_ids}")

        return removed_ids

    def clear(self):
        self._db_table_group.clear()
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
        right_ids = self._db_table_group.get_right_ids(left_entity, field_name, left_entity_ids
        )
        self.remove(right_ids)
        logging.info(f"Cascade remove {right_ids} from {left_entity} {field_name}")

    # observer methods

    def _on_related_ids_to_be_cleared_from_cache(
        self, left_entity: EntityEnum, left_ids: list[int]
    ):
        if left_entity != EntityEnum.Entity:
            return
        self.get.cache_clear()
        [self._cache.pop(id, None) for id in left_ids]
