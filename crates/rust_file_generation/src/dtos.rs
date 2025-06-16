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
