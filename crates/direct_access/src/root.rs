pub mod dtos;
pub mod root_controller;
mod use_cases;

use common_entities::root::Root;
use crate::RepositoryTrait;

pub trait RootRepositoryTrait : RepositoryTrait<Root> {}