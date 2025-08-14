use common::types::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRustFilesDto {
    pub only_existing: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateRustFilesDto {
    pub file_ids: Vec<EntityId>,
    pub root_path: String,
    pub prefix: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateRustFilesReturnDto {
    pub files: Vec<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateRustCodeDto {
    pub file_id: EntityId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateRustCodeReturnDto {
    pub generated_code: String,
    pub timestamp: String,
}
