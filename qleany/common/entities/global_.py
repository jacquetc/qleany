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
class Global(IEntity):
    id_: int
    language: str
    application_name: str
    organisation_name: str
    organisation_domain: str

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
                    field_name="language",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="application_name",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="organisation_name",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
                FieldInfo(
                    field_name="organisation_domain",
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False,
                ),
            ],
            relationships=[
                RelationshipInfo(
                    left_entity=EntityEnum.Root,
                    left_entity_name="Root",
                    right_entity=EntityEnum.Global,
                    right_entity_name="Global",
                    field_name="global_",
                    relationship_type=RelationshipType.OneToOne,
                    relationship_strength=RelationshipStrength.Strong,
                    relationship_direction=RelationshipDirection.Backward,
                    relationship_cardinality=RelationshipCardinality.One,
                )
            ],
        )
