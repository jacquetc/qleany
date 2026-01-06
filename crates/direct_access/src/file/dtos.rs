use serde::{Deserialize, Serialize};
use std::convert::From;

use common::entities::File;
use common::types::EntityId;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileDto {
    pub id: EntityId,
    pub name: String,
    pub relative_path: String,
    pub group: String,
    pub template_name: String,
    pub feature: Option<EntityId>,
    pub entity: Option<EntityId>,
    pub use_case: Option<EntityId>,
}

impl From<FileDto> for File {
    fn from(file_dto: FileDto) -> Self {
        File {
            id: file_dto.id,
            name: file_dto.name,
            relative_path: file_dto.relative_path,
            group: file_dto.group,
            template_name: file_dto.template_name,
            feature: file_dto.feature,
            entity: file_dto.entity,
            use_case: file_dto.use_case,
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
            feature: file_dto.feature,
            entity: file_dto.entity,
            use_case: file_dto.use_case,
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
            feature: file.feature,
            entity: file.entity,
            use_case: file.use_case,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFileDto {
    pub name: String,
    pub relative_path: String,
    pub group: String,
    pub template_name: String,
    pub feature: Option<EntityId>,
    pub entity: Option<EntityId>,
    pub use_case: Option<EntityId>,
}

impl From<CreateFileDto> for File {
    fn from(create_file_dto: CreateFileDto) -> Self {
        File {
            id: 0,
            name: create_file_dto.name,
            relative_path: create_file_dto.relative_path,
            group: create_file_dto.group,
            template_name: create_file_dto.template_name,
            feature: create_file_dto.feature,
            entity: create_file_dto.entity,
            use_case: create_file_dto.use_case,
        }
    }
}

impl From<&CreateFileDto> for File {
    fn from(create_file_dto: &CreateFileDto) -> Self {
        File {
            id: 0,
            name: create_file_dto.name.clone(),
            relative_path: create_file_dto.relative_path.to_string(),
            group: create_file_dto.group.clone(),
            template_name: create_file_dto.template_name.clone(),
            feature: create_file_dto.feature,
            entity: create_file_dto.entity,
            use_case: create_file_dto.use_case,
        }
    }
}

impl From<File> for CreateFileDto {
    fn from(file: File) -> Self {
        CreateFileDto {
            name: file.name,
            relative_path: file.relative_path,
            group: file.group,
            template_name: file.template_name,
            feature: file.feature,
            entity: file.entity,
            use_case: file.use_case,
        }
    }
}
