from dataclasses import dataclass
from qleany.common.entities.entity_enums import EntitySchema, EntityEnum, FieldInfo, FieldType, RelationshipInfo, RelationshipType, RelationshipStrength, RelationshipDirection, RelationshipCardinality

@dataclass
class Field:
    id: int
    name: str
    type_: str
    entity: int | None
    is_nullable: bool
    is_primary_key: bool
    is_list: bool
    strong: bool
    list_model: bool
    list_model_displayed_field: str | None

    @classmethod
    def _schema(cls) -> EntitySchema:
        return EntitySchema(
            entity_name=cls.__name__,
            fields=[
                FieldInfo(
                    field_name='id',
                    field_type=FieldType.Integer,
                    is_primary_key=True,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='name',
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='type_',
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='entity',
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True
                ),
                FieldInfo(
                    field_name='is_nullable',
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='is_primary_key',
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='is_list',
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='strong',
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='list_model',
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='list_model_displayed_field',
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False
                )
            ],
            relationships=[
                RelationshipInfo(
                    left_entity=EntityEnum.Field,
                    left_entity_name='Field',
                    right_entity=EntityEnum.Entity,
                    right_entity_name='Entity',
                    field_name='entity',
                    relationship_type=RelationshipType.OneToOne,
                    relationship_strength=RelationshipStrength.Weak,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.One
                )
            ]
        )
