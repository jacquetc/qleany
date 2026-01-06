use common::entities::{Entity, Field, FieldType, RelationshipType};
use common::types::EntityId;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldDto {
    pub id: EntityId,
    pub name: String,
    pub field_type: FieldType,
    pub entity: Option<EntityId>,
    pub relationship: RelationshipType,
    pub required: bool,
    pub single_model: bool,
    pub strong: bool,
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
            relationship: field_dto.relationship,
            required: field_dto.required,
            single_model: field_dto.single_model,
            strong: field_dto.strong,
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
            relationship: field_dto.relationship.clone(),
            required: field_dto.required,
            single_model: field_dto.single_model,
            strong: field_dto.strong,
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
            relationship: field.relationship,
            required: field.required,
            single_model: field.single_model,
            strong: field.strong,
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
    pub relationship: RelationshipType,
    pub required: bool,
    pub single_model: bool,
    pub strong: bool,
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
            relationship: create_field_dto.relationship,
            required: create_field_dto.required,
            single_model: create_field_dto.single_model,
            strong: create_field_dto.strong,
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
            relationship: create_field_dto.relationship.clone(),
            required: create_field_dto.required,
            single_model: create_field_dto.single_model,
            strong: create_field_dto.strong,
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
            relationship: field.relationship,
            required: field.required,
            single_model: field.single_model,
            strong: field.strong,
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
