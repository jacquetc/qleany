import sqlite3
from typing import Sequence

import stringcase

from qleany.common.entities.entity_enums import RelationshipInfo


class OneToManyOrderedAssociator:
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
            f"previous INTEGER, next INTEGER, "
            f"{self._junction_table_left_entity_foreign_key_name} INTEGER NOT NULL, "
            f"{self._junction_table_right_entity_foreign_key_name} INTEGER NOT NULL UNIQUE, "
            f"FOREIGN KEY ({self._junction_table_left_entity_foreign_key_name}) REFERENCES {self._left_entity_foreign_table_name} (id) ON DELETE RESTRICT, "
            f"FOREIGN KEY ({self._junction_table_right_entity_foreign_key_name}) REFERENCES {self._right_entity_foreign_table_name} (id) ON DELETE RESTRICT, "
            f"UNIQUE ({self._junction_table_right_entity_foreign_key_name}), "  # Enforce uniqueness on the right entity foreign key
            f"UNIQUE ({self._junction_table_left_entity_foreign_key_name}, {self._junction_table_right_entity_foreign_key_name}));"
        )

    def get_right_ids(self, left_entity_id: int) -> Sequence[int]:
        connection = self._db_connection
        query_str = (
            f"WITH RECURSIVE ordered_relationships(id, {self._junction_table_right_entity_foreign_key_name}, row_number) AS ("
            f"  SELECT id, {self._junction_table_right_entity_foreign_key_name}, 1"
            f"  FROM {self._junction_table_name}"
            f"  WHERE previous IS NULL AND {self._junction_table_left_entity_foreign_key_name} = ?"
            f"  UNION ALL"
            f"  SELECT deo.id, deo.{self._junction_table_right_entity_foreign_key_name}, o_r.row_number + 1"
            f"  FROM {self._junction_table_name} deo"
            f"  JOIN ordered_relationships o_r ON deo.previous = o_r.id "
            f"  AND {self._junction_table_left_entity_foreign_key_name} = ?"
            f")"
            f"SELECT {self._junction_table_right_entity_foreign_key_name} FROM ordered_relationships ORDER BY row_number"
        )

        cursor = connection.cursor()
        cursor.execute(query_str, (left_entity_id, left_entity_id))
        right_entity_ids = [row[0] for row in cursor.fetchall()]
        return right_entity_ids

    def update_right_ids(
        self, left_entity_id: int, right_entity_ids: Sequence[int]
    ) -> dict:
        connection = self._db_connection
        cursor = connection.cursor()

        # Fetch current associations
        query_str = (
            f"SELECT id, {self._junction_table_right_entity_foreign_key_name}, previous, next "
            f"FROM {self._junction_table_name} "
            f"WHERE {self._junction_table_left_entity_foreign_key_name} = ?"
        )
        cursor.execute(query_str, (left_entity_id,))
        current_associations = cursor.fetchall()

        added_relationships: list[dict[str, int]] = []
        deleted_relationships: list[dict[str, int | None]] = []

        # Create a list of current EntityShadow objects
        current_shadows = [
            {
                "junction_table_id": row[0],
                "entity_id": row[1],
                "previous": row[2],
                "next": row[3],
                "create": False,
                "remove": False,
                "common": False,
                "new_previous": 0,
                "new_next": 0,
                "update_previous_or_next": False,
            }
            for row in current_associations
        ]

        # Create a list of new EntityShadow objects
        new_shadows = [
            {
                "junction_table_id": 0,
                "entity_id": entity_id,
                "previous": 0,
                "next": 0,
                "create": True,
                "remove": False,
                "common": False,
                "new_previous": 0,
                "new_next": 0,
                "update_previous_or_next": False,
            }
            for entity_id in right_entity_ids
        ]

        # Mark common entities
        for current_shadow in current_shadows:
            for new_shadow in new_shadows:
                if current_shadow["entity_id"] == new_shadow["entity_id"]:
                    current_shadow["common"] = True
                    new_shadow["common"] = True
                    new_shadow["junction_table_id"] = current_shadow[
                        "junction_table_id"
                    ]
                    new_shadow["previous"] = current_shadow["previous"]
                    new_shadow["next"] = current_shadow["next"]
                    new_shadow["create"] = False

        # Mark entities to be removed
        for current_shadow in current_shadows:
            if not current_shadow["common"]:
                current_shadow["remove"] = True

        # Merge shadows and update previous/next pointers
        merged_shadows = new_shadows + [
            shadow for shadow in current_shadows if shadow["remove"]
        ]
        for i, shadow in enumerate(merged_shadows):
            if not shadow["remove"]:
                shadow["new_previous"] = (
                    merged_shadows[i - 1]["entity_id"] if i > 0 else 0
                )
                shadow["new_next"] = (
                    merged_shadows[i + 1]["entity_id"]
                    if i < len(merged_shadows) - 1
                    else 0
                )
                if (
                    shadow["previous"] != shadow["new_previous"]
                    or shadow["next"] != shadow["new_next"]
                ):
                    shadow["update_previous_or_next"] = True

        # Apply changes to the database
        for shadow in merged_shadows:
            if shadow["create"]:
                # Delete any existing association for the right entity
                delete_existing_query = (
                    f"DELETE FROM {self._junction_table_name} WHERE "
                    f"{self._junction_table_right_entity_foreign_key_name} = ?"
                )
                cursor.execute(delete_existing_query, (shadow["entity_id"],))
                deleted_relationships.append(
                    {
                        "left_entity_id": None,  # Previous left entity ID is unknown
                        "right_entity_id": shadow["entity_id"],
                    }
                )

                query_str = (
                    f"INSERT INTO {self._junction_table_name} "
                    f"({self._junction_table_left_entity_foreign_key_name}, {self._junction_table_right_entity_foreign_key_name}, previous, next) "
                    f"VALUES (?, ?, ?, ?)"
                )
                cursor.execute(
                    query_str,
                    (
                        left_entity_id,
                        shadow["entity_id"],
                        shadow["new_previous"] if shadow["new_previous"] != 0 else None,
                        shadow["new_next"] if shadow["new_next"] != 0 else None,
                    ),
                )
                added_relationships.append(
                    {
                        "left_entity_id": left_entity_id,
                        "right_entity_id": shadow["entity_id"],
                    }
                )
            elif shadow["remove"]:
                query_str = f"DELETE FROM {self._junction_table_name} WHERE id = ?"
                cursor.execute(query_str, (shadow["junction_table_id"],))
                deleted_relationships.append(
                    {
                        "left_entity_id": left_entity_id,
                        "right_entity_id": shadow["entity_id"],
                    }
                )
            elif shadow["update_previous_or_next"]:
                query_str = (
                    f"UPDATE {self._junction_table_name} "
                    f"SET previous = ?, next = ? "
                    f"WHERE id = ?"
                )
                cursor.execute(
                    query_str,
                    (
                        shadow["new_previous"] if shadow["new_previous"] != 0 else None,
                        shadow["new_next"] if shadow["new_next"] != 0 else None,
                        shadow["junction_table_id"],
                    ),
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
        final_added_relationships = added_relationships_grouped

        # transform deleted_relationships into group of relationships by the left_entity_id
        deleted_relationships_grouped = {}
        for relationship in deleted_relationships:
            if relationship["left_entity_id"] is None:
                left_entity_id = -1  # left entity ID is unknown
            else:
                left_entity_id = relationship["left_entity_id"]
            if left_entity_id not in deleted_relationships_grouped:
                deleted_relationships_grouped[left_entity_id] = []
            deleted_relationships_grouped[left_entity_id].append(
                relationship["right_entity_id"]
            )
        final_deleted_relationships = deleted_relationships_grouped

        return {
            "left_entity_name": self._relationship.left_entity_name,
            "left_entity_field_name": self._field_name,
            "right_entity_name": self._relationship.right_entity_name,
            "added_relationships": final_added_relationships,
            "deleted_relationships": final_deleted_relationships,
        }
