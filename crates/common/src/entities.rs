use crate::entities::Order::Ordered;
use crate::types::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Root {
    pub id: EntityId,
    pub manifest_absolute_path: String,
    pub global: EntityId,
    pub entities: Vec<EntityId>,
    pub features: Vec<EntityId>,
    pub files: Vec<EntityId>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub only_for_heritage: bool,
    pub parent: Option<EntityId>,
    pub allow_direct_access: bool,
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
pub struct File {
    pub id: EntityId,
    pub name: String,
    pub group: String,
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
    pub field_type: DtoFieldType,
    pub is_nullable: bool,
    pub is_list: bool,
    pub enum_name: Option<String>,
    pub enum_values: Option<Vec<String>>,
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
    pub enum_name: Option<String>,
    pub enum_values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Relationship {
    pub id: EntityId,
    pub left_entity: EntityId,
    pub right_entity: EntityId,
    pub field_name: String,
    pub relationship_type: RelationshipType,
    pub strength: Strength,
    pub direction: Direction,
    pub cardinality: Cardinality,
    pub order: Option<Order>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DtoFieldType {
    Boolean,
    Integer,
    UInteger,
    Float,
    String,
    Uuid,
    DateTime,
    Enum,
}

impl Default for DtoFieldType {
    fn default() -> Self {
        DtoFieldType::String
    }
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
    Entity,
    Enum,
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

impl Default for RelationshipType {
    fn default() -> Self {
        RelationshipType::ManyToMany
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Strength {
    Weak,
    Strong,
}

impl Default for Strength {
    fn default() -> Self {
        Strength::Strong
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Forward
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Cardinality {
    ZeroOrOne,
    One,
    ZeroOrMore,
    OneOrMore,
}

impl Default for Cardinality {
    fn default() -> Self {
        Cardinality::ZeroOrMore
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Order {
    Ordered,
    Unordered,
}

impl Default for Order {
    fn default() -> Self {
        Ordered
    }
}
