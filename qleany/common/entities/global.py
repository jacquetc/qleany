from dataclasses import dataclass
from qleany.common.entities.entity_enums import EntitySchema, EntityEnum, FieldInfo, FieldType, RelationshipInfo, RelationshipType, RelationshipStrength, RelationshipDirection, RelationshipCardinality

@dataclass
class Global:
    id: int
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
                    field_name='id',
                    field_type=FieldType.Integer,
                    is_primary_key=True,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='language',
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='application_name',
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='organisation_name',
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False
                ),
                FieldInfo(
                    field_name='organisation_domain',
                    field_type=FieldType.String,
                    is_primary_key=False,
                    has_relationship=False
                )
            ],
            relationships=[]
        )