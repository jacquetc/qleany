
use std::convert::From;

use common::entities::Root;
use common::entities::EntityId;

#[derive(Debug, Clone, PartialEq)]
pub struct RootDto {
    pub id: EntityId,
    pub global: EntityId,
    pub entities: Vec<EntityId>,
    pub features: Vec<EntityId>,
}

impl From<RootDto> for Root {
    fn from(root_dto: RootDto) -> Self {
        Root {
            id: root_dto.id,
            global: root_dto.global,
            entities: root_dto.entities,
            features: root_dto.features,
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
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateRootDto {
    pub global: EntityId,
    pub entities: Vec<EntityId>,
    pub features: Vec<EntityId>,
}

impl From<CreateRootDto> for Root {
    fn from(create_root_dto: CreateRootDto) -> Self {
        Root {
            id: 0,
            global: create_root_dto.global,
            entities: create_root_dto.entities,
            features: create_root_dto.features,
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
        }
    }
}

impl From<Root> for CreateRootDto {
    fn from(root: Root) -> Self {
        CreateRootDto {
            global: root.global,
            entities: root.entities,
            features: root.features,
        }
    }
}

pub use common::direct_access::root::RootRelationshipField;

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveRootRelationshipsDto {
    pub field: RootRelationshipField,
    pub ids_to_remove: Vec<EntityId>,
}