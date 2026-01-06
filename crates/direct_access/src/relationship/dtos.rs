use common::entities::{Cardinality, Direction, Order, Relationship, RelationshipType, Strength};
use common::types::EntityId;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipDto {
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

impl From<RelationshipDto> for Relationship {
    fn from(relationship_dto: RelationshipDto) -> Self {
        Relationship {
            id: relationship_dto.id,
            left_entity: relationship_dto.left_entity,
            right_entity: relationship_dto.right_entity,
            field_name: relationship_dto.field_name,
            relationship_type: relationship_dto.relationship_type,
            strength: relationship_dto.strength,
            direction: relationship_dto.direction,
            cardinality: relationship_dto.cardinality,
            order: relationship_dto.order,
        }
    }
}

impl From<&RelationshipDto> for Relationship {
    fn from(relationship_dto: &RelationshipDto) -> Self {
        Relationship {
            id: relationship_dto.id,
            left_entity: relationship_dto.left_entity,
            right_entity: relationship_dto.right_entity,
            field_name: relationship_dto.field_name.clone(),
            relationship_type: relationship_dto.relationship_type.clone(),
            strength: relationship_dto.strength.clone(),
            direction: relationship_dto.direction.clone(),
            cardinality: relationship_dto.cardinality.clone(),
            order: relationship_dto.order.clone(),
        }
    }
}

impl From<Relationship> for RelationshipDto {
    fn from(relationship: Relationship) -> Self {
        RelationshipDto {
            id: relationship.id,
            left_entity: relationship.left_entity,
            right_entity: relationship.right_entity,
            field_name: relationship.field_name,
            relationship_type: relationship.relationship_type,
            strength: relationship.strength,
            direction: relationship.direction,
            cardinality: relationship.cardinality,
            order: relationship.order,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateRelationshipDto {
    pub left_entity: EntityId,
    pub right_entity: EntityId,
    pub field_name: String,
    pub relationship_type: RelationshipType,
    pub strength: Strength,
    pub direction: Direction,
    pub cardinality: Cardinality,
    pub order: Option<Order>,
}

impl From<CreateRelationshipDto> for Relationship {
    fn from(create_relationship_dto: CreateRelationshipDto) -> Self {
        Relationship {
            id: 0,
            left_entity: create_relationship_dto.left_entity,
            right_entity: create_relationship_dto.right_entity,
            field_name: create_relationship_dto.field_name,
            relationship_type: create_relationship_dto.relationship_type,
            strength: create_relationship_dto.strength,
            direction: create_relationship_dto.direction,
            cardinality: create_relationship_dto.cardinality,
            order: create_relationship_dto.order,
        }
    }
}

impl From<&CreateRelationshipDto> for Relationship {
    fn from(create_relationship_dto: &CreateRelationshipDto) -> Self {
        Relationship {
            id: 0,
            left_entity: create_relationship_dto.left_entity,
            right_entity: create_relationship_dto.right_entity,
            field_name: create_relationship_dto.field_name.clone(),
            relationship_type: create_relationship_dto.relationship_type.clone(),
            strength: create_relationship_dto.strength.clone(),
            direction: create_relationship_dto.direction.clone(),
            cardinality: create_relationship_dto.cardinality.clone(),
            order: create_relationship_dto.order.clone(),
        }
    }
}

impl From<Relationship> for CreateRelationshipDto {
    fn from(relationship: Relationship) -> Self {
        CreateRelationshipDto {
            left_entity: relationship.left_entity,
            right_entity: relationship.right_entity,
            field_name: relationship.field_name,
            relationship_type: relationship.relationship_type,
            strength: relationship.strength,
            direction: relationship.direction,
            cardinality: relationship.cardinality,
            order: relationship.order,
        }
    }
}

pub use common::direct_access::relationship::RelationshipRelationshipField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipRelationshipDto {
    pub id: EntityId,
    pub field: RelationshipRelationshipField,
    pub right_ids: Vec<EntityId>,
}
