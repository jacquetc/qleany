from dataclasses import dataclass
from qleany.common.entities.entity_enums import EntitySchema, EntityEnum, FieldInfo, FieldType, RelationshipInfo, RelationshipType, RelationshipStrength, RelationshipDirection, RelationshipCardinality

@dataclass
class Feature:
    id: int
    name: str
    description: str
    use_cases: list[int]

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
                    field_name='description',
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='use_cases',
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True
                )
            ],
            relationships=[
                RelationshipInfo(
                    left_entity=EntityEnum.Feature,
                    left_entity_name='Feature',
                    right_entity=EntityEnum.UseCase,
                    right_entity_name='UseCase',
                    field_name='use_cases',
                    relationship_type=RelationshipType.OneToMany,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.ManyOrdered
                )
            ]
        )