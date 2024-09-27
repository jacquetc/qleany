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
class UseCase(IEntity):
    id_: int
    name: str
    validator: bool
    entities: list[int]
    undoable: bool
    dto_in: int | None
    dto_out: int | None

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
                    field_name="validator",
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="entities",
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True,
                ),
                FieldInfo(
                    field_name="undoable",
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="dto_in",
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True,
                ),
                FieldInfo(
                    field_name="dto_out",
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True,
                ),
            ],
            relationships=[
                RelationshipInfo(
                    left_entity=EntityEnum.UseCase,
                    left_entity_name="UseCase",
                    right_entity=EntityEnum.Entity,
                    right_entity_name="Entity",
                    field_name="entities",
                    relationship_type=RelationshipType.ManyToMany,
                    relationship_strength=RelationshipStrength.Weak,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.ManyOrdered,
                ),
                RelationshipInfo(
                    left_entity=EntityEnum.UseCase,
                    left_entity_name="UseCase",
                    right_entity=EntityEnum.Dto,
                    right_entity_name="Dto",
                    field_name="dto_in",
                    relationship_type=RelationshipType.OneToOne,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.One,
                ),
                RelationshipInfo(
                    left_entity=EntityEnum.UseCase,
                    left_entity_name="UseCase",
                    right_entity=EntityEnum.Dto,
                    right_entity_name="Dto",
                    field_name="dto_out",
                    relationship_type=RelationshipType.OneToOne,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.One,
                ),
                RelationshipInfo(
                    left_entity=EntityEnum.Feature,
                    left_entity_name="Feature",
                    right_entity=EntityEnum.UseCase,
                    right_entity_name="UseCase",
                    field_name="use_cases",
                    relationship_type=RelationshipType.OneToMany,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Backward,
                    relationship_cardinality=RelationshipCardinality.ManyOrdered,
                ),
            ],
        )
