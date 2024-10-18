import sqlite3
from typing import Optional

import stringcase

from qleany.common.entities.entity_enums import RelationshipInfo


class OneToOneAssociator:
    def __init__(
        self, relationship: RelationshipInfo, db_connection: sqlite3.Connection
    ):
        self._relationship = relationship
        self._db_connection = db_connection
        self._field_name = relationship.field_name

        left_entity_name = relationship.left_entity_name
        right_entity_name = relationship.right_entity_name

        self._junction_table_name = (
            f"{left_entity_name}_{relationship.field_name}_{right_entity_name}_junction"
        )
        self._junction_table_left_entity_foreign_key_name = f"{left_entity_name}_id"
        self._left_entity_foreign_table_name = stringcase.snakecase(left_entity_name)
        self._junction_table_right_entity_foreign_key_name = f"{right_entity_name}_id"
        self._right_entity_foreign_table_name = stringcase.snakecase(right_entity_name)

    def get_table_creation_sql(self):
        return (
            f"CREATE TABLE IF NOT EXISTS {self._junction_table_name} "
            f"(id INTEGER PRIMARY KEY AUTOINCREMENT, "
            f"{self._junction_table_left_entity_foreign_key_name} INTEGER NOT NULL, "
            f"{self._junction_table_right_entity_foreign_key_name} INTEGER NOT NULL, "
            f"FOREIGN KEY ({self._junction_table_left_entity_foreign_key_name}) REFERENCES {self._left_entity_foreign_table_name}(id) ON DELETE CASCADE, "
            f"FOREIGN KEY ({self._junction_table_right_entity_foreign_key_name}) REFERENCES {self._right_entity_foreign_table_name}(id) ON DELETE CASCADE, "
            f"UNIQUE ({self._junction_table_right_entity_foreign_key_name}), "  # Enforce uniqueness on the right entity foreign key
            f"UNIQUE ({self._junction_table_left_entity_foreign_key_name}, {self._junction_table_right_entity_foreign_key_name}))"
        )

    def get_right_id(self, left_entity_id: int) -> Optional[int]:
        connection = self._db_connection
        cursor = connection.cursor()
        query = (
            f"SELECT {self._junction_table_right_entity_foreign_key_name} FROM {self._junction_table_name} "
            f"WHERE {self._junction_table_left_entity_foreign_key_name} = ?"
        )
        cursor.execute(query, (left_entity_id,))
        result = cursor.fetchone()
        return result[0] if result else None

    def update_right_id(self, left_entity_id: int, right_entity_id: int) -> dict:
        connection = self._db_connection
        cursor = connection.cursor()

        deleted_relationships = []
        added_relationships = []

        if right_entity_id == 0:
            delete_query = (
                f"DELETE FROM {self._junction_table_name} WHERE "
                f"{self._junction_table_left_entity_foreign_key_name} = ?"
            )
            cursor.execute(delete_query, (left_entity_id,))
            deleted_relationships.append(
                {
                    "left_entity_id": left_entity_id,
                    "right_entity_id": self.get_right_id(left_entity_id),
                }
            )
        else:
            # Delete any existing association for the right entity
            delete_existing_query = (
                f"DELETE FROM {self._junction_table_name} WHERE "
                f"{self._junction_table_right_entity_foreign_key_name} = ?"
            )
            cursor.execute(delete_existing_query, (right_entity_id,))
            deleted_relationships.append(
                {
                    "left_entity_id": None,  # Previous left entity ID is unknown
                    "right_entity_id": right_entity_id,
                }
            )

            select_query = (
                f"SELECT {self._junction_table_right_entity_foreign_key_name} FROM {self._junction_table_name} "
                f"WHERE {self._junction_table_left_entity_foreign_key_name} = ?"
            )
            cursor.execute(select_query, (left_entity_id,))
            result = cursor.fetchone()

            if result:
                update_query = (
                    f"UPDATE {self._junction_table_name} SET "
                    f"{self._junction_table_right_entity_foreign_key_name} = ? "
                    f"WHERE {self._junction_table_left_entity_foreign_key_name} = ?"
                )
                cursor.execute(update_query, (right_entity_id, left_entity_id))
            else:
                insert_query = (
                    f"INSERT INTO {self._junction_table_name} ("
                    f"{self._junction_table_left_entity_foreign_key_name}, "
                    f"{self._junction_table_right_entity_foreign_key_name}) VALUES (?, ?)"
                )
                cursor.execute(insert_query, (left_entity_id, right_entity_id))

            added_relationships.append(
                {"left_entity_id": left_entity_id, "right_entity_id": right_entity_id}
            )

        # transform added_relationships into group of relationships by the left_entity_id
        added_relationships_grouped = {}
        for relationship in added_relationships:
            left_entity_id = relationship["left_entity_id"]
            if left_entity_id not in added_relationships_grouped:
                added_relationships_grouped[left_entity_id] = []
            added_relationships_grouped[left_entity_id].append(
                relationship["right_entity_id"]
            )
        added_relationships = added_relationships_grouped

        # transform deleted_relationships into group of relationships by the left_entity_id
        deleted_relationships_grouped = {}
        for relationship in deleted_relationships:
            left_entity_id = relationship["left_entity_id"]
            if left_entity_id not in deleted_relationships_grouped:
                deleted_relationships_grouped[left_entity_id] = []
            deleted_relationships_grouped[left_entity_id].append(
                relationship["right_entity_id"]
            )
        deleted_relationships = deleted_relationships_grouped

        return {
            "left_entity_name": self._relationship.left_entity_name,
            "left_entity_field_name": self._field_name,
            "right_entity_name": self._relationship.right_entity_name,
            "added_relationships": added_relationships,
            "deleted_relationships": deleted_relationships,
        }
