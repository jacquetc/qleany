use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct RootDto{
    pub id: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct CreateRootDto{
    pub entities: Vec<RootDto>,
    pub owner_id: Option<u64>,
    pub position: Option<u64>,
}