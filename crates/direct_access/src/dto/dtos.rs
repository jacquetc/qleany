use common::entities::Dto;
use common::types::EntityId;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DtoDto {
    pub id: EntityId,
    pub name: String,
    pub fields: Vec<EntityId>,
}

impl From<DtoDto> for Dto {
    fn from(dto_dto: DtoDto) -> Self {
        Dto {
            id: dto_dto.id,
            name: dto_dto.name,
            fields: dto_dto.fields,
        }
    }
}

impl From<&DtoDto> for Dto {
    fn from(dto_dto: &DtoDto) -> Self {
        Dto {
            id: dto_dto.id,
            name: dto_dto.name.clone(),
            fields: dto_dto.fields.clone(),
        }
    }
}

impl From<Dto> for DtoDto {
    fn from(dto: Dto) -> Self {
        DtoDto {
            id: dto.id,
            name: dto.name,
            fields: dto.fields,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateDtoDto {
    pub name: String,
    pub fields: Vec<EntityId>,
}

impl From<CreateDtoDto> for Dto {
    fn from(create_dto_dto: CreateDtoDto) -> Self {
        Dto {
            id: 0,
            name: create_dto_dto.name,
            fields: create_dto_dto.fields,
        }
    }
}

impl From<&CreateDtoDto> for Dto {
    fn from(create_dto_dto: &CreateDtoDto) -> Self {
        Dto {
            id: 0,
            name: create_dto_dto.name.clone(),
            fields: create_dto_dto.fields.clone(),
        }
    }
}

impl From<Dto> for CreateDtoDto {
    fn from(dto: Dto) -> Self {
        CreateDtoDto {
            name: dto.name,
            fields: dto.fields,
        }
    }
}

pub use common::direct_access::dto::DtoRelationshipField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DtoRelationshipDto {
    pub id: EntityId,
    pub field: DtoRelationshipField,
    pub right_ids: Vec<EntityId>,
}
