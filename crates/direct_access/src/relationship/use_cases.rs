pub(super) mod create_relationship_multi_uc;
pub(super) mod create_relationship_uc;
pub(super) mod get_relationship_multi_uc;
pub(super) mod get_relationship_relationship_uc;
pub(super) mod get_relationship_uc;
pub(super) mod remove_relationship_multi_uc;
pub(super) mod remove_relationship_uc;
pub(super) mod set_relationship_relationship_uc;
pub(super) mod update_relationship_multi_uc;
pub(super) mod update_relationship_uc;

use anyhow::Result;
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::Relationship;
use common::types::EntityId;

pub(in crate::relationship) trait RelationshipUnitOfWorkFactoryTrait:
    Send + Sync
{
    fn create(&self) -> Box<dyn RelationshipUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Relationship", action = "Create")]
#[macros::uow_action(entity = "Relationship", action = "CreateMulti")]
#[macros::uow_action(entity = "Relationship", action = "Get")]
#[macros::uow_action(entity = "Relationship", action = "GetMulti")]
#[macros::uow_action(entity = "Relationship", action = "Update")]
#[macros::uow_action(entity = "Relationship", action = "UpdateMulti")]
#[macros::uow_action(entity = "Relationship", action = "Delete")]
#[macros::uow_action(entity = "Relationship", action = "DeleteMulti")]
#[macros::uow_action(entity = "Relationship", action = "GetRelationship")]
#[macros::uow_action(entity = "Relationship", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Relationship", action = "SetRelationship")]
#[macros::uow_action(entity = "Relationship", action = "SetRelationshipMulti")]
pub(in crate::relationship) trait RelationshipUnitOfWorkTrait:
    CommandUnitOfWork
{
}

pub(in crate::relationship) trait RelationshipUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn RelationshipUnitOfWorkROTrait>;
}

#[macros::uow_action(entity = "Relationship", action = "GetRO")]
#[macros::uow_action(entity = "Relationship", action = "GetMultiRO")]
#[macros::uow_action(entity = "Relationship", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Relationship", action = "GetRelationshipsFromRightIdsRO")]
pub(in crate::relationship) trait RelationshipUnitOfWorkROTrait:
    QueryUnitOfWork
{
}
