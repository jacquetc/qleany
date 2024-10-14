from qleany.common.persistence.repositories.use_case_repository import (
    UseCaseRepository,
)
from qleany.common.direct_access.feature.i_feature_repository import (
    IFeatureRepository,
)
from qleany.common.entities.entity_enums import EntityEnum
from qleany.common.entities.feature import Feature
from functools import lru_cache
from qleany.common.direct_access.common.repository.repository_observer import (
    RepositoryObserver,
    RepositorySubject,
)
import logging


class FeatureRepository(IFeatureRepository):

    def __init__(self, use_case_repository: UseCaseRepository):
        super().__init__()
        self._database = Database(db_context)
        self._use_case_repository = use_case_repository

        self._use_case_repository.attach(self)

        self._cache = {}

    @lru_cache(maxsize=None)
    def get(self, db_connection: IDbConnection, ids: list[int]) -> list[Feature]:
        cached_entities = [self._cache[id] for id in ids if id in self._cache]
        missing_ids = [id for id in ids if id not in self._cache]
        if missing_ids:
            db_entities = self._database.get(db_connection, missing_ids)
            for entity in db_entities:
                self._cache[entity.id_] = entity
            cached_entities.extend(db_entities)
        return cached_entities

    def get_all(self, db_connection: IDbConnection) -> list[Feature]:
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
        self, db_connection: IDbConnection, entities: list[Feature]
    ) -> list[Feature]:
        created_entities = self._database.create(entities)
        for entity in created_entities:
            self._cache[entity.id_] = entity
        self.get.cache_clear()

        self._notify_created([entity.id_ for entity in created_entities])

        logging.info(f"Created {created_entities}")

        return created_entities

    def update(
        self, db_connection: IDbConnection, entities: list[Feature]
    ) -> list[Feature]:
        updated_entities = self._database.update(entities)
        for entity in updated_entities:
            if entity.id_ in self._cache:
                self._cache[entity.id_] = entity
        self.get.cache_clear()

        self._notify_updated([entity.id_ for entity in updated_entities])

        logging.info(f"Updated {updated_entities}")

        return updated_entities

    def remove(self, db_connection: IDbConnection, ids: list[int]) -> list[int]:
        # cascade remove for strong relationships
        self._use_case_repository.cascade_remove(
            db_connection, "Feature", "use_cases", ids
        )

        removed_ids = self._database.remove(ids)
        for id in removed_ids:
            if id in self._cache:
                del self._cache[id]
        self.get.cache_clear()

        # signals all repos depending of this repo
        for relationship in Feature._schema().relationships:
            if relationship.relationship_direction == RelationshipDirection.Backward:
                left_ids = self._database.get_left_ids(
                    db_connection,
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
        field_name: str,
        left_entity_ids: list[int],
    ):
        right_ids = self._database.get_right_ids(
            db_connection, left_entity, field_name, left_entity_ids
        )
        self.remove(right_ids)
        logging.info(f"Cascade remove {right_ids} from {left_entity} {field_name}")
