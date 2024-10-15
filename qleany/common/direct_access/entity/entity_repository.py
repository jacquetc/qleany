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
from qleany.common.direct_access.entity.i_entity_db_table_group import (
    IEntityDbTableGroup,
)

from qleany.common.direct_access.entity.i_entity_repository import (
    IEntityRepository,
)
from qleany.common.entities.entity_enums import (
    EntityEnum,
    RelationshipCardinality,
    RelationshipDirection,
    RelationshipStrength,
    RelationshipType,
)
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

    def get(self, ids: Sequence[int]) -> Sequence[Entity]:
        db_entities = self._db_table_group.get(ids)
        return db_entities

    def get_all(self) -> Sequence[Entity]:
        db_entities = self._db_table_group.get_all()
        return db_entities

    def get_all_ids(self) -> Sequence[int]:
        db_entities = self.get_all()
        return [entity.id_ for entity in db_entities]

    def create(self, entities: Sequence[Entity], owner_id: int, position: int) -> Sequence[Entity]:

        created_entities = self._db_table_group.create(entities)

        # create relationship
        for relationship in Entity._schema().relationships:
            if (
                relationship.relationship_direction == RelationshipDirection.Backward
                and relationship.relationship_strength == RelationshipStrength.Strong
            ):
                # get entity name from relationship
                repository_name = f"{relationship.right_entity_name}Repository"

                # create repository from factory
                owner_repository = self._repository_factory.create(
                    repository_name, self._db_connection
                )

                # get owner entity
                owner = owner_repository.get([owner_id])[0]

                new_ids = [entity.id_ for entity in created_entities]

                owner_field = getattr(owner, relationship.field_name)
                # add new entities to owner
                if relationship.relationship_type == RelationshipType.OneToOne:
                    if owner_field != new_ids[0]:
                        # remove old entity
                        if owner_field != 0:
                            self.remove([owner_field])

                        setattr(owner, relationship.field_name, new_ids[0])
                        owner_repository.update([owner])

                elif relationship.relationship_type == RelationshipType.OneToMany and relationship.relationship_cardinality == RelationshipCardinality.ManyUnordered:
                    if owner_field is None:
                        owner_field = []
                    owner_field += new_ids
                    setattr(owner, relationship.field_name, owner_field)
                    owner_repository.update([owner])

                elif relationship.relationship_type == RelationshipType.OneToMany and relationship.relationship_cardinality == RelationshipCardinality.ManyOrdered:
                    if owner_field is None:
                        owner_field = []

                    if position == -1: 
                        owner_field += new_ids
                    elif position == 0:
                        owner_field = new_ids + owner_field
                    elif position > 0 and position < len(owner_field):
                        owner_field = owner_field[:position] + new_ids + owner_field[position:]
                    elif position == len(owner_field):
                        owner_field += new_ids
                    elif position > len(owner_field):
                        owner_field = new_ids + owner_field
                    elif position < -1:
                        raise ValueError(f"Invalid position {position}")
                    else:
                        raise ValueError(f"Invalid position {position}")

                    setattr(owner, relationship.field_name, owner_field)
                    owner_repository.update([owner])
                    
                elif relationship.relationship_type == RelationshipType.ManyToMany and relationship.relationship_cardinality == RelationshipCardinality.ManyUnordered:
                    if owner_field is None:
                        owner_field = []
                    owner_field += new_ids
                    setattr(owner, relationship.field_name, owner_field)
                    owner_repository.update([owner])

                elif relationship.relationship_type == RelationshipType.ManyToMany and relationship.relationship_cardinality == RelationshipCardinality.ManyOrdered:
                    raise NotImplementedError()
                else:
                    # unreachable
                    raise NotImplementedError()
                
                break

        # notify
        self._messenger.notify(
            "Entity", "created", {"ids": [entity.id_ for entity in created_entities]}
        )

        logging.info(f"Created {created_entities}")

        return created_entities

    def update(self, entities: Sequence[Entity]) -> Sequence[Entity]:
        updated_entities = self._db_table_group.update(entities)

        self._messenger.notify(
            "Entity", "updated", {"ids": [entity.id_ for entity in updated_entities]}
        )

        logging.info(f"Updated {updated_entities}")

        return updated_entities

    def remove(self, ids: Sequence[int]) -> Sequence[int]:

        # cascade remove relationships
        for relationship in Entity._schema().relationships:
            if (
                relationship.relationship_direction == RelationshipDirection.Forward
                and relationship.relationship_strength == RelationshipStrength.Strong
            ):
                # get entity name from relationship
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

        self._messenger.notify("Entity", "removed", {"ids": removed_ids})

        logging.info(f"Removed {removed_ids}")

        return removed_ids

    def clear(self):
        self._db_table_group.clear()

        self._messenger.notify("Entity", "cleared", {})

        logging.info("Cache cleared")

    def exists(self, id_: int) -> bool:
        return self._db_table_group.exists(id_)
    