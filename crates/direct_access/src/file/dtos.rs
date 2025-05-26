use serde::{Deserialize, Serialize};
use std::convert::From;

use common::entities::{File, Group};
use common::types::EntityId;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileDto {
    pub id: EntityId,
    pub name: String,
    pub group: Group,
}

impl From<FileDto> for File {
    fn from(file_dto: FileDto) -> Self {
        File {
            id: file_dto.id,
            name: file_dto.name,
            group: file_dto.group,
        }
    }
}

impl From<&FileDto> for File {
    fn from(file_dto: &FileDto) -> Self {
        File {
            id: file_dto.id,
            name: file_dto.name.clone(),
            group: file_dto.group.clone(),
        }
    }
}

impl From<File> for FileDto {
    fn from(file: File) -> Self {
        FileDto {
            id: file.id,
            name: file.name,
            group: file.group,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFileDto {
    pub name: String,
    pub group: Group,
}

impl From<CreateFileDto> for File {
    fn from(create_file_dto: CreateFileDto) -> Self {
        File {
            id: 0,
            name: create_file_dto.name,
            group: create_file_dto.group,
        }
    }
}

impl From<&CreateFileDto> for File {
    fn from(create_file_dto: &CreateFileDto) -> Self {
        File {
            id: 0,
            name: create_file_dto.name.clone(),
            group: create_file_dto.group.clone(),
        }
    }
}

impl From<File> for CreateFileDto {
    fn from(file: File) -> Self {
        CreateFileDto {
            name: file.name,
            group: file.group,
        }
    }
}
