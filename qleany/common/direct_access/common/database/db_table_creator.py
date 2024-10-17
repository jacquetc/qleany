from typing import List, Sequence, Type

from qleany.common.direct_access.common.database.interfaces.i_db_connection import (
    IDbConnection,
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
    FieldType,
    RelationshipCardinality,
    RelationshipDirection,
    RelationshipType,
)
from qleany.common.entities.i_entity import IEntity


class DbTableCreator:
    def __init__(self, db_connection: IDbConnection):
        self._db_connection = db_connection
        self._entity_sqls: List[str] = []
        self._junction_sqls: List[str] = []

    def add_tables(self, entities: Sequence[Type[IEntity]]):
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
            # renaming id:
            field_name = field.field_name
            if field_name == "id_":
                field_name = "id"
            # sql string:
            field_sql = f"{field_name} {self._get_sql_type(field.field_type)}"
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
        connection = self._db_connection.connection()
        for relationship in schema.relationships:
            if relationship.relationship_direction == RelationshipDirection.Backward:
                junction_sql: str = ""
                if relationship.relationship_cardinality == RelationshipCardinality.One:
                    junction_sql = OneToOneAssociator(
                        relationship, connection
                    ).get_table_creation_sql()
                elif (
                    relationship.relationship_cardinality
                    == RelationshipCardinality.ManyOrdered
                    and relationship.relationship_type == RelationshipType.OneToMany
                ):
                    junction_sql = OneToManyOrderedAssociator(
                        relationship, connection
                    ).get_table_creation_sql()
                elif (
                    relationship.relationship_cardinality
                    == RelationshipCardinality.ManyUnordered
                    and relationship.relationship_type == RelationshipType.OneToMany
                ):
                    junction_sql = OneToManyUnorderedAssociator(
                        relationship, connection
                    ).get_table_creation_sql()
                elif (
                    relationship.relationship_cardinality
                    == RelationshipCardinality.ManyOrdered
                    and relationship.relationship_type == RelationshipType.ManyToMany
                ):
                    raise ValueError(
                        "Many to Many Ordered relationships are not supported"
                    )
                elif (
                    relationship.relationship_cardinality
                    == RelationshipCardinality.ManyUnordered
                    and relationship.relationship_type == RelationshipType.ManyToMany
                ):
                    junction_sql = ManyToManyUnorderedAssociator(
                        relationship, connection
                    ).get_table_creation_sql()

                if junction_sql:
                    self._junction_sqls.append(junction_sql)

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
