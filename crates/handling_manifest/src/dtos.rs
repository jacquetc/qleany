use common::types::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadDto {
    pub manifest_path: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadReturnDto {
    pub root_id: u64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveDto {
    pub manifest_path: String,
}
