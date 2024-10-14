import sqlite3
from typing import Sequence

from qleany.common.direct_access.common.database.interfaces.i_db_table_group import (
    IDbTableGroup,
)
from qleany.common.direct_access.common.database.many_to_many_unordered_associator import (
    ManyToManyUnorderedAssociator,
)
from qleany.common.direct_access.common.database.one_to_many_ordered_associator import (
    OneToManyOrderedAssociator,
)
from qleany.common.direct_access.common.database.one_to_many_unordered_associator import (
    OneToManyUnorderedAssociator,
)
from qleany.common.direct_access.common.database.one_to_one_associator import (
    OneToOneAssociator,
)
from qleany.common.entities.entity_enums import (
    RelationshipCardinality,
    RelationshipDirection,
    RelationshipInfo,
    RelationshipType,
)
from qleany.common.entities.i_entity import IEntity


class SqliteDbTableGroup(IDbTableGroup):

    def __init__(self, entity_type: type[IEntity], db_connection: sqlite3.Connection):
        self._entity_type = entity_type
        self._db_connection = db_connection

    def get(self, ids: Sequence[int]) -> Sequence[IEntity]:
        # This method should return a list of entities from the database with the given ids.
        cursor = self._db_connection.cursor()
        cursor.execute(
            f"SELECT * FROM {self._entity_type.__name__} WHERE id IN ({','.join('?' for _ in ids)})",
            ids,
        )

        entities = []
        for row in cursor.fetchall():
            entity = self._entity_type(*row)

            for relationship in entity._schema().relationships:
                if relationship.relationship_direction == RelationshipDirection.Forward:
                    right_ids = self.get_right_ids(relationship, entity.id_)
                    if relationship.relationship_type == RelationshipType.OneToOne:
                        if right_ids:
                            setattr(entity, relationship.field_name, right_ids[0])
                        else:
                            setattr(entity, relationship.field_name, None)
                    else:
                        setattr(entity, relationship.field_name, right_ids)

            entities.append(entity)

        return entities

    def get_all(self) -> Sequence[IEntity]:
        # This method should return a list of all entities from the database.
        cursor = self._db_connection.cursor()
        cursor.execute(f"SELECT * FROM {self._entity_type.__name__}")

        entities = []
        for row in cursor.fetchall():
            entity = self._entity_type(*row)

            for relationship in entity._schema().relationships:
                if relationship.relationship_direction == RelationshipDirection.Forward:
                    right_ids = self.get_right_ids(relationship, entity.id_)
                    if relationship.relationship_type == RelationshipType.OneToOne:
                        if right_ids:
                            setattr(entity, relationship.field_name, right_ids[0])
                        else:
                            setattr(entity, relationship.field_name, None)
                    else:
                        setattr(entity, relationship.field_name, right_ids)

            entities.append(entity)

        return entities

    def get_all_ids(self) -> Sequence[int]:
        # This method should return a list of all entity ids from the database.
        cursor = self._db_connection.cursor()
        cursor.execute(f"SELECT id_ FROM {self._entity_type.__name__}")
        return [row[0] for row in cursor.fetchall()]

    def create(self, entities: Sequence[IEntity]) -> Sequence[IEntity]:
        # This method should create the given entities in the database and return a list of the created entities.
        cursor = self._db_connection.cursor()

        new_ids = []
        for entity in entities:
            cursor.execute(
                f"INSERT INTO {self._entity_type.__name__} ({','.join('?' for _ in self._fields_without_relationships())}) VALUES {','.join('?' for _ in self._fields_without_relationships())}",
                self._convert_entity_to_tuple(entity),
            )
            # retrieve the id of the created entity
            last_row_id = cursor.lastrowid
            new_ids.append(last_row_id)

            if last_row_id is None:
                raise Exception("Error creating entity")

            for relationship in entity._schema().relationships:
                if relationship.relationship_direction == RelationshipDirection.Forward:
                    right_ids = getattr(entity, relationship.field_name)
                    if relationship.relationship_type == RelationshipType.OneToOne:
                        if right_ids:
                            OneToOneAssociator(
                                relationship, self._db_connection
                            ).update_right_id(last_row_id, right_ids)
                    else:
                        if right_ids:
                            if relationship.relationship_type == RelationshipType.OneToMany:
                                if relationship.relationship_cardinality == RelationshipCardinality.ManyOrdered:
                                    OneToManyOrderedAssociator(
                                        relationship, self._db_connection
                                    ).update_right_ids(last_row_id, right_ids)
                                elif relationship.relationship_cardinality == RelationshipCardinality.ManyUnordered:
                                    OneToManyUnorderedAssociator(
                                        relationship, self._db_connection
                                    ).update_right_ids(last_row_id, right_ids)
                            elif relationship.relationship_type == RelationshipType.ManyToMany:
                                if relationship.relationship_cardinality == RelationshipCardinality.ManyUnordered:
                                    ManyToManyUnorderedAssociator(
                                        relationship, self._db_connection
                                    ).update_right_ids(last_row_id, right_ids)

        for entity, id_ in zip(entities, new_ids):
            entity.id_ = id_

        return entities

    def _fields_without_relationships(self):
        return [
            field
            for field in self._entity_type._schema().fields
            if field.has_relationship is False
        ]

    def _fields_with_relationships(self):
        return [
            field
            for field in self._entity_type._schema().fields
            if field.has_relationship is True
        ]

    def _convert_entity_to_tuple(self, entity: IEntity) -> tuple:
        return tuple(
            getattr(entity, field.field_name)
            for field in self._fields_without_relationships()
        )

    def update(self, entities: Sequence[IEntity]) -> Sequence[IEntity]:
        # This method should update the given entities in the database and return a list of the updated entities.
        cursor = self._db_connection.cursor()
        cursor.executemany(
            f"UPDATE {self._entity_type.__name__} SET {','.join(f'{field.field_name}=?' for field in entities[0]._schema().fields)} WHERE id=?",
            [
                self._convert_entity_to_tuple(entity) + (entity.id_,)
                for entity in entities
            ],
        )
        
        for entity in entities:
            for relationship in entity._schema().relationships:
                if relationship.relationship_direction == RelationshipDirection.Forward:
                    right_ids = getattr(entity, relationship.field_name)
                    if relationship.relationship_type == RelationshipType.OneToOne:
                        if right_ids:
                            OneToOneAssociator(
                                relationship, self._db_connection
                            ).update_right_id(entity.id_, right_ids)
                    else:
                        if right_ids:
                            if relationship.relationship_type == RelationshipType.OneToMany:
                                if relationship.relationship_cardinality == RelationshipCardinality.ManyOrdered:
                                    OneToManyOrderedAssociator(
                                        relationship, self._db_connection
                                    ).update_right_ids(entity.id_, right_ids)
                                elif relationship.relationship_cardinality == RelationshipCardinality.ManyUnordered:
                                    OneToManyUnorderedAssociator(
                                        relationship, self._db_connection
                                    ).update_right_ids(entity.id_, right_ids)
                            elif relationship.relationship_type == RelationshipType.ManyToMany:
                                if relationship.relationship_cardinality == RelationshipCardinality.ManyUnordered:
                                    ManyToManyUnorderedAssociator(
                                        relationship, self._db_connection
                                    ).update_right_ids(entity.id_, right_ids)

        return entities

    def remove(self, ids: Sequence[int]) -> Sequence[int]:
        # This method should remove the entities with the given ids from the database.
        cursor = self._db_connection.cursor()
        cursor.execute(
            f"DELETE FROM {self._entity_type.__name__} WHERE id IN ({','.join('?' for _ in ids)})",
            ids,
        )

        return ids

    def clear(self):
        # This method should remove all entities from the database.
        cursor = self._db_connection.cursor()
        cursor.execute(f"DELETE FROM {self._entity_type.__name__}")

    def get_left_ids(
        self, relationship: RelationshipInfo, right_id: int
    ) -> Sequence[int]:

        return []

    def get_right_ids(
        self, relationship: RelationshipInfo, left_id: int
    ) -> Sequence[int]:

        if relationship.relationship_direction == RelationshipDirection.Forward:
            if relationship.relationship_type == RelationshipType.OneToOne:
                right_id = OneToOneAssociator(
                    relationship, self._db_connection
                ).get_right_id(left_id)
                return [right_id] if right_id is not None else []

            elif (
                relationship.relationship_type == RelationshipType.OneToMany
                and relationship.relationship_cardinality
                == RelationshipCardinality.ManyOrdered
            ):
                return OneToManyOrderedAssociator(
                    relationship, self._db_connection
                ).get_right_ids(left_id)
            elif (
                relationship.relationship_type == RelationshipType.OneToMany
                and relationship.relationship_cardinality
                == RelationshipCardinality.ManyUnordered
            ):
                return OneToManyUnorderedAssociator(
                    relationship, self._db_connection
                ).get_right_ids(left_id)
            elif (
                relationship.relationship_type == RelationshipType.ManyToMany
                and relationship.relationship_cardinality
                == RelationshipCardinality.ManyUnordered
            ):
                return ManyToManyUnorderedAssociator(
                    relationship, self._db_connection
                ).get_right_ids(left_id)
            else:  # ManyToManyOrdered unimplemented
                # unreacheable
                return []

        else:
            # unreacheable
            return []

    def update_right_ids(
        self, relationship: RelationshipInfo, left_id: int, right_ids: Sequence[int]
    ) -> Sequence[int]:

        if relationship.relationship_direction == RelationshipDirection.Forward:
            if relationship.relationship_type == RelationshipType.OneToOne:
                OneToOneAssociator(relationship, self._db_connection).update_right_id(
                    left_id, right_ids[0] if right_ids else 0
                )
                return right_ids

            elif (
                relationship.relationship_type == RelationshipType.OneToMany
                and relationship.relationship_cardinality
                == RelationshipCardinality.ManyOrdered
            ):
                OneToManyOrderedAssociator(
                    relationship, self._db_connection
                ).update_right_ids(left_id, right_ids)
                return right_ids
            elif (
                relationship.relationship_type == RelationshipType.OneToMany
                and relationship.relationship_cardinality
                == RelationshipCardinality.ManyUnordered
            ):
                OneToManyUnorderedAssociator(
                    relationship, self._db_connection
                ).update_right_ids(left_id, right_ids)
                return right_ids
            elif (
                relationship.relationship_type == RelationshipType.ManyToMany
                and relationship.relationship_cardinality
                == RelationshipCardinality.ManyUnordered
            ):
                ManyToManyUnorderedAssociator(
                    relationship, self._db_connection
                ).update_right_ids(left_id, right_ids)
                return right_ids
            else:  # ManyToManyOrdered unimplemented
                # unreacheable
                return []
        else:
            # unreacheable
            return []
