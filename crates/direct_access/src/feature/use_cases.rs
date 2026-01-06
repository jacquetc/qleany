pub(super) mod create_feature_multi_uc;
pub(super) mod create_feature_uc;
pub(super) mod get_feature_multi_uc;
pub(super) mod get_feature_relationship_uc;
pub(super) mod get_feature_uc;
pub(super) mod remove_feature_multi_uc;
pub(super) mod remove_feature_uc;
pub(super) mod set_feature_relationship_uc;
pub(super) mod update_feature_multi_uc;
pub(super) mod update_feature_uc;

use anyhow::Result;
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::Feature;
use common::types::EntityId;

pub(in crate::feature) trait FeatureUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn FeatureUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Feature", action = "Create")]
#[macros::uow_action(entity = "Feature", action = "CreateMulti")]
#[macros::uow_action(entity = "Feature", action = "Get")]
#[macros::uow_action(entity = "Feature", action = "GetMulti")]
#[macros::uow_action(entity = "Feature", action = "Update")]
#[macros::uow_action(entity = "Feature", action = "UpdateMulti")]
#[macros::uow_action(entity = "Feature", action = "Delete")]
#[macros::uow_action(entity = "Feature", action = "DeleteMulti")]
#[macros::uow_action(entity = "Feature", action = "GetRelationship")]
#[macros::uow_action(entity = "Feature", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Feature", action = "SetRelationship")]
#[macros::uow_action(entity = "Feature", action = "SetRelationshipMulti")]
pub(in crate::feature) trait FeatureUnitOfWorkTrait: CommandUnitOfWork {}

pub(in crate::feature) trait FeatureUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn FeatureUnitOfWorkROTrait>;
}

#[macros::uow_action(entity = "Feature", action = "GetRO")]
#[macros::uow_action(entity = "Feature", action = "GetMultiRO")]
#[macros::uow_action(entity = "Feature", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Feature", action = "GetRelationshipsFromRightIdsRO")]
pub(in crate::feature) trait FeatureUnitOfWorkROTrait: QueryUnitOfWork {}
