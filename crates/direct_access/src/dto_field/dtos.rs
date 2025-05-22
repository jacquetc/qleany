use common::entities::DtoField;
use common::entities::DtoFieldType;
use common::types::EntityId;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DtoFieldDto {
    pub id: EntityId,
    pub name: String,
    pub field_type: DtoFieldType,
    pub is_nullable: bool,
    pub is_list: bool,
}

impl From<DtoFieldDto> for DtoField {
    fn from(dto_field_dto: DtoFieldDto) -> Self {
        DtoField {
            id: dto_field_dto.id,
            name: dto_field_dto.name,
            field_type: dto_field_dto.field_type,
            is_nullable: dto_field_dto.is_nullable,
            is_list: dto_field_dto.is_list,
        }
    }
}

impl From<&DtoFieldDto> for DtoField {
    fn from(dto_field_dto: &DtoFieldDto) -> Self {
        DtoField {
            id: dto_field_dto.id,
            name: dto_field_dto.name.clone(),
            field_type: dto_field_dto.field_type.clone(),
            is_nullable: dto_field_dto.is_nullable,
            is_list: dto_field_dto.is_list,
        }
    }
}

impl From<DtoField> for DtoFieldDto {
    fn from(dto_field: DtoField) -> Self {
        DtoFieldDto {
            id: dto_field.id,
            name: dto_field.name,
            field_type: dto_field.field_type,
            is_nullable: dto_field.is_nullable,
            is_list: dto_field.is_list,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateDtoFieldDto {
    pub name: String,
    pub field_type: DtoFieldType,
    pub is_nullable: bool,
    pub is_list: bool,
}

impl From<CreateDtoFieldDto> for DtoField {
    fn from(create_dto_field_dto: CreateDtoFieldDto) -> Self {
        DtoField {
            id: 0,
            name: create_dto_field_dto.name,
            field_type: create_dto_field_dto.field_type,
            is_nullable: create_dto_field_dto.is_nullable,
            is_list: create_dto_field_dto.is_list,
        }
    }
}

impl From<&CreateDtoFieldDto> for DtoField {
    fn from(create_dto_field_dto: &CreateDtoFieldDto) -> Self {
        DtoField {
            id: 0,
            name: create_dto_field_dto.name.clone(),
            field_type: create_dto_field_dto.field_type.clone(),
            is_nullable: create_dto_field_dto.is_nullable,
            is_list: create_dto_field_dto.is_list,
        }
    }
}

impl From<DtoField> for CreateDtoFieldDto {
    fn from(dto_field: DtoField) -> Self {
        CreateDtoFieldDto {
            name: dto_field.name,
            field_type: dto_field.field_type,
            is_nullable: dto_field.is_nullable,
            is_list: dto_field.is_list,
        }
    }
}
