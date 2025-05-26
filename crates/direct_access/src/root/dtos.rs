use common::entities::Root;
use common::types::EntityId;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RootDto {
    pub id: EntityId,
    pub global: EntityId,
    pub entities: Vec<EntityId>,
    pub features: Vec<EntityId>,
    pub files: Vec<EntityId>,
}

impl From<RootDto> for Root {
    fn from(root_dto: RootDto) -> Self {
        Root {
            id: root_dto.id,
            global: root_dto.global,
            entities: root_dto.entities,
            features: root_dto.features,
            files: root_dto.files,
        }
    }
}

impl From<&RootDto> for Root {
    fn from(root_dto: &RootDto) -> Self {
        Root {
            id: root_dto.id,
            global: root_dto.global,
            entities: root_dto.entities.clone(),
            features: root_dto.features.clone(),
            files: root_dto.files.clone(),
        }
    }
}

impl From<Root> for RootDto {
    fn from(root: Root) -> Self {
        RootDto {
            id: root.id,
            global: root.global,
            entities: root.entities,
            features: root.features,
            files: root.files,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateRootDto {
    pub global: EntityId,
    pub entities: Vec<EntityId>,
    pub features: Vec<EntityId>,
    pub files: Vec<EntityId>,
}

impl From<CreateRootDto> for Root {
    fn from(create_root_dto: CreateRootDto) -> Self {
        Root {
            id: 0,
            global: create_root_dto.global,
            entities: create_root_dto.entities,
            features: create_root_dto.features,
            files: create_root_dto.files,
        }
    }
}

impl From<&CreateRootDto> for Root {
    fn from(create_root_dto: &CreateRootDto) -> Self {
        Root {
            id: 0,
            global: create_root_dto.global,
            entities: create_root_dto.entities.clone(),
            features: create_root_dto.features.clone(),
            files: create_root_dto.files.clone(),
        }
    }
}

impl From<Root> for CreateRootDto {
    fn from(root: Root) -> Self {
        CreateRootDto {
            global: root.global,
            entities: root.entities,
            features: root.features,
            files: root.files,
        }
    }
}

pub use common::direct_access::root::RootRelationshipField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootRelationshipDto {
    pub id: EntityId,
    pub field: RootRelationshipField,
    pub right_ids: Vec<EntityId>,
}
