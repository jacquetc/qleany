from typing import List, Type

from qleany.common.entities.entity_enums import (
    FieldType,
    RelationshipCardinality,
    RelationshipDirection,
    RelationshipType,
)
from qleany.common.entities.i_entity import IEntity
from qleany.common.persistence.database.db_connection import DbConnection
from qleany.common.persistence.database.many_to_many_unordered_associator import (
    ManyToManyUnorderedAssociator,
)
from qleany.common.persistence.database.one_to_many_ordered_associator import (
    OneToManyOrderedAssociator,
)
from qleany.common.persistence.database.one_to_many_unordered_associator import (
    OneToManyUnorderedAssociator,
)
from qleany.common.persistence.database.one_to_one_associator import (
    OneToOneAssociator,
)


class DBTableCreator:

    def __init__(self, db_connection: DbConnection):
        self._db_connection = db_connection
        self._entity_sqls: List[str] = []
        self._junction_sqls: List[str] = []

    def add_tables(self, entities: List[Type[IEntity]]):
        for entity_class in entities:
            self._add_entity_table_sql(entity_class)
            self._add_junction_tables_sql(entity_class)

    def _add_entity_table_sql(self, entity_class: Type[IEntity]):

        schema = entity_class._schema()
        table_name = schema.entity_name
        fields_sql = []

        for field in schema.fields:
            if field.has_relationship:
                continue

            field_sql = f"{field.field_name} {self._get_sql_type(field.field_type)}"
            if field.is_primary_key:
                field_sql += " PRIMARY KEY"
            fields_sql.append(field_sql)

        create_table_sql = f"CREATE TABLE {table_name} ({', '.join(fields_sql)});"
        self._entity_sqls.append(create_table_sql)

    def _get_sql_type(self, field_type: FieldType) -> str:
        if field_type == FieldType.Bool:
            return "BOOLEAN"
        elif field_type == FieldType.Integer:
            return "INTEGER"
        elif field_type == FieldType.Float:
            return "REAL"
        elif field_type == FieldType.String:
            return "TEXT"
        elif field_type == FieldType.Uuid:
            return "TEXT"  # UUIDs can be stored as TEXT in SQLite
        elif field_type == FieldType.DateTime:
            return "TEXT"  # DateTime can be stored as TEXT in SQLite
        else:
            raise ValueError(f"Unsupported field type: {field_type}")

    def _add_junction_tables_sql(self, entity_class: Type[IEntity]):
        schema = entity_class._schema()
        for relationship in schema.relationships:
            if relationship.relationship_direction == RelationshipDirection.Backward:
                if relationship.relationship_cardinality == RelationshipCardinality.One:
                    OneToOneAssociator(relationship).get_table_creation_sql()
                elif relationship.relationship_cardinality == RelationshipCardinality.ManyOrdered and relationship.relationship_type == RelationshipType.OneToMany:
                    OneToManyOrderedAssociator(relationship).get_table_creation_sql()
                elif relationship.relationship_cardinality == RelationshipCardinality.ManyUnordered and relationship.relationship_type == RelationshipType.OneToMany:
                    OneToManyUnorderedAssociator(relationship).get_table_creation_sql()
                elif relationship.relationship_cardinality == RelationshipCardinality.ManyOrdered and relationship.relationship_type == RelationshipType.OneToMany:
                    raise ValueError("Many to Many Ordered relationships are not supported")
                elif relationship.relationship_cardinality == RelationshipCardinality.ManyUnordered and relationship.relationship_type == RelationshipType.OneToMany:
                    ManyToManyUnorderedAssociator(relationship).get_table_creation_sql()


    def create_empty_database(self):
        try:
            with self._db_connection.connection() as conn:
                cursor = conn.cursor()

                # Create the entity tables in the database
                for table in self._entity_sqls:
                    cursor.execute(table)

                # Create the junction tables in the database
                for table in self._junction_sqls:
                    cursor.execute(table)

                # Execute additional PRAGMA statements for optimization
                optimization_pragmas = [
                    "PRAGMA case_sensitive_like=true",
                    "PRAGMA journal_mode=MEMORY",
                    "PRAGMA temp_store=MEMORY",
                    "PRAGMA locking_mode=NORMAL",
                    "PRAGMA synchronous=OFF",
                    "PRAGMA recursive_triggers=ON",
                    "PRAGMA foreign_keys=ON",
                ]

                for pragma in optimization_pragmas:
                    cursor.execute(pragma)

                conn.commit()

        except Exception as e:
            raise RuntimeError(f"Error creating database: {str(e)}")
