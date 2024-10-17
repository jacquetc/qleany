
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EntityEnum {
    Root,
    Entity,
    Field,
    Feature,
    Dto,
    DtoField,
    UseCase,
    Relationship,
    Global,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldType {
    Bool,
    Integer,
    Float,
    String,
    Uuid,
    DateTime,
    Entity,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToMany,
    ManyToOne,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RelationshipStrength {
    Strong,
    Weak,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RelationshipDirection {
    Forward,
    Backward,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RelationshipCardinality {
    One,
    ManyOrdered,
    ManyUnordered,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RelationshipInfo<'a> {
    pub left_entity: EntityEnum,
    pub left_entity_name: &'a str,
    pub right_entity: EntityEnum,
    pub right_entity_name: &'a str,
    pub field_name: &'a str,
    pub relationship_type: RelationshipType,
    pub relationship_strength: RelationshipStrength,
    pub relationship_direction: RelationshipDirection,
    pub relationship_cardinality: RelationshipCardinality,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FieldInfo<'a> {
    pub field_name: &'a str,
    pub field_type: FieldType,
    pub is_primary_key: bool,
    pub has_relationship: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntitySchema<'a> {
    pub entity_name: &'a str,
    pub fields: Vec<FieldInfo<'a>>,
    pub relationships: Vec<RelationshipInfo<'a>>,
}
