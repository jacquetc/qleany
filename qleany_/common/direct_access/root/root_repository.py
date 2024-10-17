import logging
from typing import Sequence

from qleany_.common.direct_access.common.database.interfaces.i_db_connection import (
    IDbConnection,
)
from qleany_.common.direct_access.common.repository.repository_factory import (
    IRepositoryFactory,
)
from qleany_.common.direct_access.common.repository.repository_messenger import (
    IMessenger,
)
from qleany_.common.direct_access.root.i_root_db_table_group import (
    IRootDbTableGroup,
)
from qleany_.common.direct_access.root.i_root_repository import (
    IRootRepository,
)
from qleany_.common.entities.entity_enums import (
    RelationshipDirection,
    RelationshipStrength,
)
from qleany_.common.entities.root import Root


class RootRepository(IRootRepository):
    def __init__(
        self,
        db_table_group: IRootDbTableGroup,
        db_connection: IDbConnection,
        repository_factory: IRepositoryFactory,
        messenger: IMessenger,
    ):
        super().__init__()
        self._db_table_group = db_table_group
        self._db_connection = db_connection
        self._repository_factory = repository_factory
        self._messenger = messenger

    def get(self, ids: Sequence[int]) -> Sequence[Root]:
        db_entities = self._db_table_group.get(ids)
        return db_entities

    def get_all(self) -> Sequence[Root]:
        db_entities = self._db_table_group.get_all()
        return db_entities

    def get_all_ids(self) -> Sequence[int]:
        db_entities = self.get_all()
        return [root.id_ for root in db_entities]

    def create(self, entities: Sequence[Root]) -> Sequence[Root]:
        created_entities = self._db_table_group.create(entities)

        self._messenger.notify(
            "Root", "created", {"ids": [root.id_ for root in created_entities]}
        )

        logging.info(f"Created {created_entities}")

        return created_entities

    def update(self, entities: Sequence[Root]) -> Sequence[Root]:
        updated_entities = self._db_table_group.update(entities)

        self._messenger.notify(
            "Root", "updated", {"ids": [root.id_ for root in updated_entities]}
        )

        logging.info(f"Updated {updated_entities}")

        return updated_entities

    def remove(self, ids: Sequence[int]) -> Sequence[int]:
        # cascade remove relationships
        for relationship in Root._schema().relationships:
            if (
                relationship.relationship_direction == RelationshipDirection.Forward
                and relationship.relationship_strength == RelationshipStrength.Strong
            ):
                # get root name from relationship
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

        self._messenger.notify("Root", "removed", {"ids": removed_ids})

        logging.info(f"Removed {removed_ids}")

        return removed_ids

    def clear(self):
        self._db_table_group.clear()

        self._messenger.notify("Root", "cleared", {})

        logging.info("Cache cleared")

    def exists(self, id_: int) -> bool:
        return self._db_table_group.exists(id_)
