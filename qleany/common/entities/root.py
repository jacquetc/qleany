from dataclasses import dataclass
from qleany.common.entities.entity_enums import EntitySchema, EntityEnum, FieldInfo, FieldType, RelationshipInfo, RelationshipType, RelationshipStrength, RelationshipDirection, RelationshipCardinality

@dataclass
class Root:
    id_: int
    global_: int
    entities: list[int]
    features: list[int]



    @classmethod
    def _schema(cls) -> EntitySchema:
        return EntitySchema(
            entity_name=cls.__name__,
            fields=[
                FieldInfo(
                    field_name='id_',
                    field_type=FieldType.Integer,
                    is_primary_key=True,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='global_',
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True
                ),
                FieldInfo(
                    field_name='entities',
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True
                ),
                FieldInfo(
                    field_name='features',
                    field_type=FieldType.Integer,
                    is_primary_key=False,
                    has_relationship=True
                )
            ],
            relationships=[
                RelationshipInfo(
                    left_entity=EntityEnum.Root,
                    left_entity_name='Root',
                    right_entity=EntityEnum.Global,
                    right_entity_name='Global',
                    field_name='global_',
                    relationship_type=RelationshipType.OneToOne,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.One
                ),
                RelationshipInfo(
                    left_entity=EntityEnum.Root,
                    left_entity_name='Root',
                    right_entity=EntityEnum.Entity,
                    right_entity_name='Entity',
                    field_name='entities',
                    relationship_type=RelationshipType.OneToMany,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.ManyOrdered
                ),
                RelationshipInfo(
                    left_entity=EntityEnum.Root,
                    left_entity_name='Root',
                    right_entity=EntityEnum.Feature,
                    right_entity_name='Feature',
                    field_name='features',
                    relationship_type=RelationshipType.OneToMany,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Forward,
                    relationship_cardinality=RelationshipCardinality.ManyOrdered
                )
            ]
        )