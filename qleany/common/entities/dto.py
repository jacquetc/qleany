from dataclasses import dataclass
from qleany.common.entities.entity_enums import (
    EntitySchema,
    EntityEnum,
    FieldInfo,
    FieldType,
    RelationshipInfo,
    RelationshipType,
    RelationshipStrength,
    RelationshipDirection,
    RelationshipCardinality,
)


@dataclass(slots=True)
class Dto:
    id_: int
    name: str
    fields: list[int]

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
                    field_name="fields",
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True,
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
                    relationship_direction=RelationshipDirection.Backward,
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
                    relationship_direction=RelationshipDirection.Backward,
                    relationship_cardinality=RelationshipCardinality.One,
                ),
            ],
        )
