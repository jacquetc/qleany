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
class DtoField(IEntity):
    id_: int = 0
    name: str = ""
    type_: str = ""
    is_nullable: bool = False
    is_list: bool = False

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
                    field_name="name",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="type_",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="is_nullable",
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="is_list",
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False,
                ),
            ],
            relationships=[
                RelationshipInfo(
                    left_entity=EntityEnum.Dto,
                    left_entity_name="Dto",
                    right_entity=EntityEnum.DtoField,
                    right_entity_name="DtoField",
                    field_name="fields",
                    relationship_type=RelationshipType.OneToMany,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Backward,
                    relationship_cardinality=RelationshipCardinality.ManyOrdered,
                )
            ],
        )
