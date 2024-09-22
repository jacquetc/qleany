from enum import Enum

class EntityEnum(Enum):
    Root = 1
    Entity = 2
    Field = 3
    Feature = 4
    Dto = 5
    DtoField = 6
    UseCase = 7
    Relationship = 8
    Global = 9

class FieldType(Enum):
    Bool = 1
    Integer = 2
    Float = 3
    String = 4
    Uuid = 5
    DateTime = 6
    Entity = 7

class RelationshipType(Enum):
    OneToOne = 1
    OneToMany = 2
    ManyToMany = 3
    ManyToOne = 4

class RelationshipStrength(Enum):
    Strong = 1
    Weak = 2

class RelationshipDirection(Enum):
    """
    RelationshipDirection
    Forward: the relationship is defined in the current entity
    Backward: the relationship is defined in the related entity
    Note: this is used to determine the name of the relationship in the related
    entity or the junction table name
    """
    Forward = 1
    Backward = 2
    
class RelationshipCardinality(Enum):
    """
    RelationshipCardinality
    One: the relationship is one-to-one
    ManyOrdered: the relationship is one-to-many and the order matters
    ManyUnordered: the relationship is one-to-many and the order does not matter
    Note: this is used to determine the name of the relationship in the related
    entity or the junction table name
    """
    One = 1
    ManyOrdered = 2
    ManyUnordered = 3

class RelationshipInfo:
    left_entity: EntityEnum
    left_entity_name: str
    right_entity: EntityEnum
    right_entity_name: str
    field_name: str
    relationship_type: RelationshipType
    relationship_strength: RelationshipStrength
    relationship_direction: RelationshipDirection
    relationship_cardinality: RelationshipCardinality

    def __init__(self, left_entity: EntityEnum, left_entity_name: str, right_entity: EntityEnum, right_entity_name: str, field_name: str, relationship_type: RelationshipType, relationship_strength: RelationshipStrength, relationship_direction: RelationshipDirection, relationship_cardinality: RelationshipCardinality):
        self.left_entity = left_entity
        self.left_entity_name = left_entity_name
        self.right_entity = right_entity
        self.right_entity_name = right_entity_name
        self.field_name = field_name
        self.relationship_type = relationship_type
        self.relationship_strength = relationship_strength
        self.relationship_direction = relationship_direction
        self.relationship_cardinality = relationship_cardinality

class FieldInfo:
    field_name: str
    field_type: FieldType
    is_primary_key: bool
    has_relationship: bool

    def __init__(self, field_name: str, field_type: FieldType, is_primary_key: bool, has_relationship: bool):
        self.field_name = field_name
        self.field_type = field_type
        self.is_primary_key = is_primary_key
        self.has_relationship = has_relationship

class EntitySchema:
    entity_name: str
    fields: list[FieldInfo]
    relationships: list[RelationshipInfo]

    def __init__(self, entity_name: str, fields: list[FieldInfo], relationships: list[RelationshipInfo]):
        self.entity_name = entity_name
        self.fields = fields
        self.relationships = relationships