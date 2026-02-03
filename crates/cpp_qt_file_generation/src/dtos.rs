use common::types::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListCppQtFilesDto {
    pub only_list_already_existing: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListCppQtFilesReturnDto {
    pub file_ids: Vec<EntityId>,
    pub file_names: Vec<String>,
    pub file_groups: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateCppQtFilesDto {
    pub file_ids: Vec<EntityId>,
    pub root_path: String,
    pub prefix: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateCppQtFilesReturnDto {
    pub files: Vec<String>,
    pub timestamp: String,
    pub duration: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateCppQtCodeDto {
    pub file_id: EntityId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateCppQtCodeReturnDto {
    pub generated_code: String,
    pub timestamp: String,
}
