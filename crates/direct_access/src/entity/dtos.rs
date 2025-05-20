use serde::{Deserialize, Serialize};
use std::convert::From;

use common::entities::Entity;
use common::types::EntityId;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EntityDto {
    pub id: EntityId,
    pub name: String,
    pub only_for_heritage: bool,
    pub parent: Option<EntityId>,
    pub fields: Vec<EntityId>,
    pub relationships: Vec<EntityId>,
}

impl From<EntityDto> for Entity {
    fn from(entity_dto: EntityDto) -> Self {
        Entity {
            id: entity_dto.id,
            name: entity_dto.name,
            only_for_heritage: entity_dto.only_for_heritage,
            parent: entity_dto.parent,
            fields: entity_dto.fields,
            relationships: entity_dto.relationships,
        }
    }
}

impl From<&EntityDto> for Entity {
    fn from(entity_dto: &EntityDto) -> Self {
        Entity {
            id: entity_dto.id,
            name: entity_dto.name.clone(),
            only_for_heritage: entity_dto.only_for_heritage,
            parent: entity_dto.parent,
            fields: entity_dto.fields.clone(),
            relationships: entity_dto.relationships.clone(),
        }
    }
}

impl From<Entity> for EntityDto {
    fn from(entity: Entity) -> Self {
        EntityDto {
            id: entity.id,
            name: entity.name,
            only_for_heritage: entity.only_for_heritage,
            parent: entity.parent,
            fields: entity.fields,
            relationships: entity.relationships,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateEntityDto {
    pub name: String,
    pub only_for_heritage: bool,
    pub parent: Option<EntityId>,
    pub fields: Vec<EntityId>,
    pub relationships: Vec<EntityId>,
}

impl From<CreateEntityDto> for Entity {
    fn from(create_entity_dto: CreateEntityDto) -> Self {
        Entity {
            id: 0,
            name: create_entity_dto.name,
            only_for_heritage: create_entity_dto.only_for_heritage,
            parent: create_entity_dto.parent,
            fields: create_entity_dto.fields,
            relationships: create_entity_dto.relationships,
        }
    }
}

impl From<&CreateEntityDto> for Entity {
    fn from(create_entity_dto: &CreateEntityDto) -> Self {
        Entity {
            id: 0,
            name: create_entity_dto.name.clone(),
            only_for_heritage: create_entity_dto.only_for_heritage,
            parent: create_entity_dto.parent,
            fields: create_entity_dto.fields.clone(),
            relationships: create_entity_dto.relationships.clone(),
        }
    }
}

impl From<Entity> for CreateEntityDto {
    fn from(entity: Entity) -> Self {
        CreateEntityDto {
            name: entity.name,
            only_for_heritage: entity.only_for_heritage,
            parent: entity.parent,
            fields: entity.fields,
            relationships: entity.relationships,
        }
    }
}

pub use common::direct_access::entity::EntityRelationshipField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityRelationshipDto {
    pub id: EntityId,
    pub field: EntityRelationshipField,
    pub right_ids: Vec<EntityId>,
}
