from dataclasses import dataclass
from qleany.common.entities.entity_enums import EntitySchema, EntityEnum, FieldInfo, FieldType, RelationshipInfo, RelationshipType, RelationshipStrength, RelationshipDirection, RelationshipCardinality

@dataclass(slots=True)
class DtoField:
    id: int
    name: str
    type_: str
    dto: int | None
    is_nullable: bool
    is_list: bool

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
                    field_name='dto',
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
                    field_name='is_list',
                    field_type=FieldType.Bool,
                    is_primary_key=False,
                    has_relationship=False
                )
            ],
            relationships=[
                RelationshipInfo(
                    left_entity=EntityEnum.DtoField,
                    left_entity_name='DtoField',
                    right_entity=EntityEnum.Dto,
                    right_entity_name='Dto',
                    field_name='dto',
                    relationship_type=RelationshipType.ManyToOne,
                    relationship_strength=RelationshipStrength.Weak,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.One
                ),
                RelationshipInfo(
                    left_entity=EntityEnum.Dto,
                    left_entity_name='Dto',
                    right_entity=EntityEnum.DtoField,
                    right_entity_name='DtoField',
                    field_name='dto_fields',
                    relationship_type=RelationshipType.OneToMany,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Backward,
                    relationship_cardinality=RelationshipCardinality.ManyOrdered
                )
            ]
        )