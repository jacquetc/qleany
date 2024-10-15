from typing import Sequence
from qleany.common.direct_access.common.database.interfaces.i_db_connection import (
    IDbConnection,
)
from qleany.common.direct_access.common.repository.repository_factory import (
    IRepositoryFactory,
)
from qleany.common.direct_access.common.repository.repository_messenger import (
    IMessenger,
)
from qleany.common.direct_access.feature.i_feature_db_table_group import (
    IFeatureDbTableGroup,
)
from qleany.common.direct_access.feature.i_feature_repository import (
    IFeatureRepository,
)
from qleany.common.entities.entity_enums import (
    EntityEnum,
    RelationshipDirection,
    RelationshipStrength,
)
import logging
from qleany.common.entities.feature import Feature
from typing import Sequence


class FeatureRepository(IFeatureRepository):

    def __init__(
        self,
        db_table_group: IFeatureDbTableGroup,
        db_connection: IDbConnection,
        repository_factory: IRepositoryFactory,
        messenger: IMessenger,
    ):
        super().__init__()
        self._db_table_group = db_table_group
        self._db_connection = db_connection
        self._repository_factory = repository_factory
        self._messenger = messenger

    def get(self, ids: Sequence[int]) -> Sequence[Feature]:
        db_entities = self._db_table_group.get(ids)
        return db_entities

    def get_all(self) -> Sequence[Feature]:
        db_entities = self._db_table_group.get_all()
        return db_entities

    def get_all_ids(self) -> Sequence[int]:
        db_entities = self.get_all()
        return [feature.id_ for feature in db_entities]

    def create(self, entities: Sequence[Feature]) -> Sequence[Feature]:
        created_entities = self._db_table_group.create(entities)

        self._messenger.notify(
            "Feature", "created", {"ids": [feature.id_ for feature in created_entities]}
        )

        logging.info(f"Created {created_entities}")

        return created_entities

    def update(self, entities: Sequence[Feature]) -> Sequence[Feature]:
        updated_entities = self._db_table_group.update(entities)

        self._messenger.notify(
            "Feature", "updated", {"ids": [feature.id_ for feature in updated_entities]}
        )

        logging.info(f"Updated {updated_entities}")

        return updated_entities

    def remove(self, ids: Sequence[int]) -> Sequence[int]:

        # cascade remove relationships
        for relationship in Feature._schema().relationships:
            if (
                relationship.relationship_direction == RelationshipDirection.Forward
                and relationship.relationship_strength == RelationshipStrength.Strong
            ):
                # get feature name from relationship
                repository_name = f"{relationship.right_entity_name}Repository"

                # create repository from factory
                repository = self._repository_factory.create(
                    repository_name, self._db_connection
                )
                
                for left_id in ids:
                    right_ids = self._db_table_group.get_right_ids(
                        relationship=relationship,
                        left_id=left_id,
                    )
                    repository.remove(
                        right_ids,
                    )

        # remove entities
        removed_ids = self._db_table_group.remove(ids)

        self._messenger.notify("Feature", "removed", {"ids": removed_ids})

        logging.info(f"Removed {removed_ids}")

        return removed_ids

    def clear(self):
        self._db_table_group.clear()

        self._messenger.notify("Feature", "cleared", {})

        logging.info("Cache cleared")
