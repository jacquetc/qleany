use common::entities::Root;
use common::types::EntityId;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RootDto {
    pub id: EntityId,
    pub manifest_absolute_path: String,
    pub global: EntityId,
    pub entities: Vec<EntityId>,
    pub features: Vec<EntityId>,
    pub files: Vec<EntityId>,
}

impl From<RootDto> for Root {
    fn from(root_dto: RootDto) -> Self {
        Root {
            id: root_dto.id,
            manifest_absolute_path: root_dto.manifest_absolute_path,
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
            manifest_absolute_path: root_dto.manifest_absolute_path.clone(),
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
            manifest_absolute_path: root.manifest_absolute_path,
            global: root.global,
            entities: root.entities,
            features: root.features,
            files: root.files,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateRootDto {
    pub manifest_absolute_path: String,
    pub global: EntityId,
    pub entities: Vec<EntityId>,
    pub features: Vec<EntityId>,
    pub files: Vec<EntityId>,
}

impl From<CreateRootDto> for Root {
    fn from(create_root_dto: CreateRootDto) -> Self {
        Root {
            id: 0,
            manifest_absolute_path: create_root_dto.manifest_absolute_path,
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
            manifest_absolute_path: create_root_dto.manifest_absolute_path.clone(),
            global: create_root_dto.global.clone(),
            entities: create_root_dto.entities.clone(),
            features: create_root_dto.features.clone(),
            files: create_root_dto.files.clone(),
        }
    }
}

impl From<Root> for CreateRootDto {
    fn from(root: Root) -> Self {
        CreateRootDto {
            manifest_absolute_path: root.manifest_absolute_path,
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
