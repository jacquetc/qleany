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
class Entity:
    id_: int
    only_for_heritage: bool
    fields: list[int]
    relationships: list[int]

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
                    field_name="only_for_heritage",
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="fields",
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True,
                ),
                FieldInfo(
                    field_name="relationships",
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True,
                ),
            ],
            relationships=[
                RelationshipInfo(
                    left_entity=EntityEnum.Entity,
                    left_entity_name="Entity",
                    right_entity=EntityEnum.Field,
                    right_entity_name="Field",
                    field_name="fields",
                    relationship_type=RelationshipType.OneToMany,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.ManyOrdered,
                ),
                RelationshipInfo(
                    left_entity=EntityEnum.Entity,
                    left_entity_name="Entity",
                    right_entity=EntityEnum.Relationship,
                    right_entity_name="Relationship",
                    field_name="relationships",
                    relationship_type=RelationshipType.OneToMany,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.ManyOrdered,
                ),
                RelationshipInfo(
                    left_entity=EntityEnum.UseCase,
                    left_entity_name="UseCase",
                    right_entity=EntityEnum.Entity,
                    right_entity_name="Entity",
                    field_name="entities",
                    relationship_type=RelationshipType.ManyToMany,
                    relationship_strength=RelationshipStrength.Weak,
                    relationship_direction=RelationshipDirection.Backward,
                    relationship_cardinality=RelationshipCardinality.ManyUnordered,
                ),
                RelationshipInfo(
                    left_entity=EntityEnum.Root,
                    left_entity_name="Root",
                    right_entity=EntityEnum.Entity,
                    right_entity_name="Entity",
                    field_name="entities",
                    relationship_type=RelationshipType.OneToMany,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Backward,
                    relationship_cardinality=RelationshipCardinality.ManyUnordered,
                ),
                RelationshipInfo(
                    left_entity=EntityEnum.Field,
                    left_entity_name="Field",
                    right_entity=EntityEnum.Entity,
                    right_entity_name="Entity",
                    field_name="entity",
                    relationship_type=RelationshipType.ManyToOne,
                    relationship_strength=RelationshipStrength.Weak,
                    relationship_direction=RelationshipDirection.Backward,
                    relationship_cardinality=RelationshipCardinality.One,
                ),
            ],
        )
