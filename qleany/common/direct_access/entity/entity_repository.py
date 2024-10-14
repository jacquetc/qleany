from typing import Sequence
from qleany.common.direct_access.common.database.interfaces.i_db_connection import IDbConnection
from qleany.common.direct_access.common.repository.repository_factory import IRepositoryFactory
from qleany.common.direct_access.common.repository.repository_messenger import IMessenger
from qleany.common.direct_access.entity.i_entity_db_table_group import IEntityDbTableGroup
from qleany.common.persistence.repositories.interfaces.i_field_repository import IFieldRepository
from qleany.common.persistence.repositories.relationship_repository import (
    RelationshipRepository, 
)

from qleany.common.direct_access.entity.i_entity_repository import (
    IEntityRepository,
)
from qleany.common.entities.entity_enums import EntityEnum, RelationshipDirection, RelationshipStrength
from functools import lru_cache
import logging
from qleany.common.entities.entity import Entity
from typing import Sequence


class EntityRepository(IEntityRepository):

    def __init__(
        self,
        db_table_group: IEntityDbTableGroup,
        db_connection: IDbConnection,
        repository_factory: IRepositoryFactory,
        messenger: IMessenger,
    ):
        super().__init__()
        self._db_table_group = db_table_group
        self._db_connection = db_connection
        self._repository_factory = repository_factory
        self._messenger = messenger
        

    def get(self, ids: Sequence[int]) -> list[Entity]:
        db_entities = self._db_table_group.get(ids)
        return db_entities

    def get_all(self) -> list[Entity]:
        db_entities = self._db_table_group.get_all()
        return db_entities

    def get_all_ids(self) -> list[int]:
        db_entities = self.get_all()
        return [entity.id_ for entity in db_entities]


    def create(
        self, entities: Sequence[Entity]
    ) -> list[Entity]:
        created_entities = self._db_table_group.create(entities)

        self._messenger.notify("Entity", "created", {"ids": [entity.id_ for entity in created_entities]})

        logging.info(f"Created {created_entities}")

        return created_entities

    def update(
        self, entities: Sequence[Entity]
    ) -> list[Entity]:
        updated_entities = self._db_table_group.update(entities)

        self._messenger.notify("Entity", "updated", {"ids": [entity.id_ for entity in updated_entities]})

        logging.info(f"Updated {updated_entities}")

        return updated_entities

    def remove(self, ids: Sequence[int]) -> list[int]:
        
        # cascade remove relationships
        for relationship in Entity._schema().relationships:
            if relationship.relationship_direction == RelationshipDirection.Forward and relationship.relationship_strength == RelationshipStrength.Strong:
                    # get entity name from relationship
                    repository_name = f"{relationship.right_entity_name}Repository"

                    # create repository from factory
                    repository = self._repository_factory.create(repository_name, self._db_connection)      

                    right_ids = self._db_table_group.get_right_ids(
                                                                    left_entity=relationship.left_entity_name,
                                                                    field_name=relationship.field_name,
                                                                    left_ids=ids,
                                                                    )
                    repository.remove(
                        right_ids,
                    )
                    
        # remove entities
        removed_ids = self._db_table_group.remove(ids)

        self._messenger.notify("Entity", "removed", {"ids": removed_ids})

        logging.info(f"Removed {removed_ids}")

        return removed_ids

    def clear(self):
        self._db_table_group.clear()
        
        self._messenger.notify("Entity", "cleared", {})

        logging.info("Cache cleared")
