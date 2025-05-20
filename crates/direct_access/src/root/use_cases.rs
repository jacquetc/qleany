pub(super) mod create_root_multi_uc;
pub(super) mod create_root_uc;
pub(super) mod get_root_multi_uc;
pub(super) mod get_root_relationship_uc;
pub(super) mod get_root_uc;
pub(super) mod remove_root_multi_uc;
pub(super) mod remove_root_uc;
pub(super) mod set_root_relationship_uc;
pub(super) mod update_root_multi_uc;
pub(super) mod update_root_uc;

use anyhow::Result;
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::Root;
use common::types::EntityId;

pub(in crate::root) trait RootUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn RootUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "Create")]
#[macros::uow_action(entity = "Root", action = "CreateMulti")]
#[macros::uow_action(entity = "Root", action = "Get")]
#[macros::uow_action(entity = "Root", action = "GetMulti")]
#[macros::uow_action(entity = "Root", action = "Update")]
#[macros::uow_action(entity = "Root", action = "UpdateMulti")]
#[macros::uow_action(entity = "Root", action = "Delete")]
#[macros::uow_action(entity = "Root", action = "DeleteMulti")]
#[macros::uow_action(entity = "Root", action = "GetRelationship")]
#[macros::uow_action(entity = "Root", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Root", action = "SetRelationship")]
#[macros::uow_action(entity = "Root", action = "SetRelationshipMulti")]
pub(in crate::root) trait RootUnitOfWorkTrait: CommandUnitOfWork {}

pub(in crate::root) trait RootUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn RootUnitOfWorkROTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetRO")]
#[macros::uow_action(entity = "Root", action = "GetMultiRO")]
#[macros::uow_action(entity = "Root", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Root", action = "GetRelationshipsFromRightIdsRO")]
pub(in crate::root) trait RootUnitOfWorkROTrait: QueryUnitOfWork {}
