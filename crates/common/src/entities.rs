use serde::{Deserialize, Serialize};
pub type EntityId = u64;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Root {
    pub id: EntityId,
    pub global: EntityId,
    pub entities: Vec<EntityId>,
    pub features: Vec<EntityId>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub only_for_heritage: bool,
    pub parent: Option<EntityId>,
    pub fields: Vec<EntityId>,
    pub relationships: Vec<EntityId>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Feature {
    pub id: EntityId,
    pub name: String,
    pub use_cases: Vec<EntityId>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UseCase {
    pub id: EntityId,
    pub name: String,
    pub validator: bool,
    pub entities: Vec<EntityId>,
    pub undoable: bool,
    pub dto_in: Option<EntityId>,
    pub dto_out: Option<EntityId>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Global {
    pub id: EntityId,
    pub language: String,
    pub application_name: String,
    pub organisation_name: String,
    pub organisation_domain: String,
    pub prefix_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Dto {
    pub id: EntityId,
    pub name: String,
    pub fields: Vec<EntityId>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DtoField {
    pub id: EntityId,
    pub name: String,
    pub field_type: String, // FieldType,
    pub is_nullable: bool,
    pub is_list: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FieldType {
    Boolean,
    Integer,
    UInteger,
    Float,
    String,
    Uuid,
    DateTime,
    Entity
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Field {
    pub id: EntityId,
    pub name: String,
    pub field_type: FieldType,
    pub entity: Option<EntityId>,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub is_list: bool,
    pub single: bool,
    pub strong: bool,
    pub ordered: bool,
    pub list_model: bool,
    pub list_model_displayed_field: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Strength {
    Weak,
    Strong
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Direction {
    Forward,
    Backward
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Cardinality {
    ZeroOrOne,
    One,
    ZeroOrMore,
    OneOrMore
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Order
{
    Ordered,
    Unordered
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Relationship {
    pub id: EntityId,
    pub left_entity_name: String,
    pub right_entity_name: String,
    pub field_name: String,
    pub relationship_type: RelationshipType,
    pub strength: Strength,
    pub direction: Direction,
    pub cardinality: Cardinality,
    pub order: Option<Order>,
}