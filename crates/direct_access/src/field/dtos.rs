use common::entities::{Entity, Field, FieldType};
use common::types::EntityId;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldDto {
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

impl From<FieldDto> for Field {
    fn from(field_dto: FieldDto) -> Self {
        Field {
            id: field_dto.id,
            name: field_dto.name,
            field_type: field_dto.field_type,
            entity: field_dto.entity,
            is_nullable: field_dto.is_nullable,
            is_primary_key: field_dto.is_primary_key,
            is_list: field_dto.is_list,
            single: field_dto.single,
            strong: field_dto.strong,
            ordered: field_dto.ordered,
            list_model: field_dto.list_model,
            list_model_displayed_field: field_dto.list_model_displayed_field,
            enum_name: field_dto.enum_name,
            enum_values: field_dto.enum_values,
        }
    }
}

impl From<&FieldDto> for Field {
    fn from(field_dto: &FieldDto) -> Self {
        Field {
            id: field_dto.id,
            name: field_dto.name.clone(),
            field_type: field_dto.field_type.clone(),
            entity: field_dto.entity,
            is_nullable: field_dto.is_nullable,
            is_primary_key: field_dto.is_primary_key,
            is_list: field_dto.is_list,
            single: field_dto.single,
            strong: field_dto.strong,
            ordered: field_dto.ordered,
            list_model: field_dto.list_model,
            list_model_displayed_field: field_dto.list_model_displayed_field.clone(),
            enum_name: field_dto.enum_name.clone(),
            enum_values: field_dto.enum_values.clone(),
        }
    }
}

impl From<Field> for FieldDto {
    fn from(field: Field) -> Self {
        FieldDto {
            id: field.id,
            name: field.name,
            field_type: field.field_type,
            entity: field.entity,
            is_nullable: field.is_nullable,
            is_primary_key: field.is_primary_key,
            is_list: field.is_list,
            single: field.single,
            strong: field.strong,
            ordered: field.ordered,
            list_model: field.list_model,
            list_model_displayed_field: field.list_model_displayed_field,
            enum_name: field.enum_name,
            enum_values: field.enum_values,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFieldDto {
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

impl From<CreateFieldDto> for Field {
    fn from(create_field_dto: CreateFieldDto) -> Self {
        Field {
            id: 0,
            name: create_field_dto.name,
            field_type: create_field_dto.field_type,
            entity: create_field_dto.entity,
            is_nullable: create_field_dto.is_nullable,
            is_primary_key: create_field_dto.is_primary_key,
            is_list: create_field_dto.is_list,
            single: create_field_dto.single,
            strong: create_field_dto.strong,
            ordered: create_field_dto.ordered,
            list_model: create_field_dto.list_model,
            list_model_displayed_field: create_field_dto.list_model_displayed_field,
            enum_name: create_field_dto.enum_name,
            enum_values: create_field_dto.enum_values,
        }
    }
}

impl From<&CreateFieldDto> for Field {
    fn from(create_field_dto: &CreateFieldDto) -> Self {
        Field {
            id: 0,
            name: create_field_dto.name.clone(),
            field_type: create_field_dto.field_type.clone(),
            entity: create_field_dto.entity,
            is_nullable: create_field_dto.is_nullable,
            is_primary_key: create_field_dto.is_primary_key,
            is_list: create_field_dto.is_list,
            single: create_field_dto.single,
            strong: create_field_dto.strong,
            ordered: create_field_dto.ordered,
            list_model: create_field_dto.list_model,
            list_model_displayed_field: create_field_dto.list_model_displayed_field.clone(),
            enum_name: create_field_dto.enum_name.clone(),
            enum_values: create_field_dto.enum_values.clone(),
        }
    }
}

impl From<Field> for CreateFieldDto {
    fn from(field: Field) -> Self {
        CreateFieldDto {
            name: field.name,
            field_type: field.field_type,
            entity: field.entity,
            is_nullable: field.is_nullable,
            is_primary_key: field.is_primary_key,
            is_list: field.is_list,
            single: field.single,
            strong: field.strong,
            ordered: field.ordered,
            list_model: field.list_model,
            list_model_displayed_field: field.list_model_displayed_field,
            enum_name: field.enum_name,
            enum_values: field.enum_values,
        }
    }
}

pub use common::direct_access::field::FieldRelationshipField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldRelationshipDto {
    pub id: EntityId,
    pub field: FieldRelationshipField,
    pub right_ids: Vec<EntityId>,
}
