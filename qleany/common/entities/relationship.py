from dataclasses import dataclass

from qleany.common.entities.entity_enums import (
    EntityEnum,
    EntitySchema,
    FieldInfo,
    FieldType,
    RelationshipCardinality,
    RelationshipDirection,
    RelationshipInfo,
    RelationshipStrength,
    RelationshipType,
)
from qleany.common.entities.i_entity import IEntity


@dataclass(slots=True)
class Relationship(IEntity):
    id_: int = 0
    left_entity_name: str = ""
    right_entity_name: str = ""
    field_name: str = ""
    relationship_type: str = ""
    strength: str = ""
    direction: str = ""
    cardinality: str = ""

    @classmethod
    def _schema(cls) -> EntitySchema:
        return EntitySchema(
            entity_name=cls.__name__,
            fields=[
                FieldInfo(
                    field_name="id_",
                    field_type=FieldType.Integer,
                    is_primary_key=True,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="left_entity_name",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="right_entity_name",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="field_name",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="relationship_type",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="strength",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="direction",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="cardinality",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
            ],
            relationships=[
                RelationshipInfo(
                    left_entity=EntityEnum.Entity,
                    left_entity_name="Entity",
                    right_entity=EntityEnum.Relationship,
                    right_entity_name="Relationship",
                    field_name="relationships",
                    relationship_type=RelationshipType.OneToMany,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Backward,
                    relationship_cardinality=RelationshipCardinality.ManyUnordered,
                )
            ],
        )
