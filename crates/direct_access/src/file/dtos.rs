use serde::{Deserialize, Serialize};
use std::convert::From;

use common::entities::{File, FileStatus};
use common::types::EntityId;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileDto {
    pub id: EntityId,
    pub name: String,
    pub relative_path: String,
    pub group: String,
    pub template_name: String,
    pub generated_code: Option<String>,
    pub status: FileStatus,
    pub feature: Option<EntityId>,
    pub entity: Option<EntityId>,
    pub use_case: Option<EntityId>,
    pub field: Option<EntityId>,
}

impl From<FileDto> for File {
    fn from(file_dto: FileDto) -> Self {
        File {
            id: file_dto.id,
            name: file_dto.name,
            relative_path: file_dto.relative_path,
            group: file_dto.group,
            template_name: file_dto.template_name,
            generated_code: file_dto.generated_code,
            status: file_dto.status,
            feature: file_dto.feature,
            entity: file_dto.entity,
            use_case: file_dto.use_case,
            field: file_dto.field,
        }
    }
}

impl From<&FileDto> for File {
    fn from(file_dto: &FileDto) -> Self {
        File {
            id: file_dto.id,
            name: file_dto.name.clone(),
            relative_path: file_dto.relative_path.to_string(),
            group: file_dto.group.clone(),
            template_name: file_dto.template_name.clone(),
            generated_code: file_dto.generated_code.clone(),
            status: file_dto.status.clone(),
            feature: file_dto.feature,
            entity: file_dto.entity,
            use_case: file_dto.use_case,
            field: file_dto.field,
        }
    }
}

impl From<File> for FileDto {
    fn from(file: File) -> Self {
        FileDto {
            id: file.id,
            name: file.name,
            relative_path: file.relative_path,
            group: file.group,
            template_name: file.template_name,
            generated_code: file.generated_code,
            status: file.status,
            feature: file.feature,
            entity: file.entity,
            use_case: file.use_case,
            field: file.field,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFileDto {
    pub name: String,
    pub relative_path: String,
    pub group: String,
    pub template_name: String,
    pub generated_code: Option<String>,
    pub status: FileStatus,
    pub feature: Option<EntityId>,
    pub entity: Option<EntityId>,
    pub use_case: Option<EntityId>,
    pub field: Option<EntityId>,
}

impl From<CreateFileDto> for File {
    fn from(create_file_dto: CreateFileDto) -> Self {
        File {
            id: 0,
            name: create_file_dto.name,
            relative_path: create_file_dto.relative_path,
            group: create_file_dto.group,
            template_name: create_file_dto.template_name,
            generated_code: create_file_dto.generated_code,
            status: create_file_dto.status,
            feature: create_file_dto.feature,
            entity: create_file_dto.entity,
            use_case: create_file_dto.use_case,
            field: create_file_dto.field,
        }
    }
}

impl From<&CreateFileDto> for File {
    fn from(create_file_dto: &CreateFileDto) -> Self {
        File {
            id: 0,
            name: create_file_dto.name.clone(),
            relative_path: create_file_dto.relative_path.clone(),
            group: create_file_dto.group.clone(),
            template_name: create_file_dto.template_name.clone(),
            generated_code: create_file_dto.generated_code.clone(),
            status: create_file_dto.status.clone(),
            feature: create_file_dto.feature,
            entity: create_file_dto.entity,
            use_case: create_file_dto.use_case,
            field: create_file_dto.field,
        }
    }
}

impl From<File> for CreateFileDto {
    fn from(file: File) -> Self {
        CreateFileDto {
            name: file.name.clone(),
            relative_path: file.relative_path.clone(),
            group: file.group.clone(),
            template_name: file.template_name.clone(),
            generated_code: file.generated_code.clone(),
            status: file.status.clone(),
            feature: file.feature,
            entity: file.entity,
            use_case: file.use_case,
            field: file.field,
        }
    }
}
