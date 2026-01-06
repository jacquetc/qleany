pub(super) mod create_field_multi_uc;
pub(super) mod create_field_uc;
pub(super) mod get_field_multi_uc;
pub(super) mod get_field_relationship_uc;
pub(super) mod get_field_uc;
pub(super) mod remove_field_multi_uc;
pub(super) mod remove_field_uc;
pub(super) mod set_field_relationship_uc;
pub(super) mod update_field_multi_uc;
pub(super) mod update_field_uc;

use anyhow::Result;
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::Field;
use common::types::EntityId;

pub(in crate::field) trait FieldUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn FieldUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Field", action = "Create")]
#[macros::uow_action(entity = "Field", action = "CreateMulti")]
#[macros::uow_action(entity = "Field", action = "Get")]
#[macros::uow_action(entity = "Field", action = "GetMulti")]
#[macros::uow_action(entity = "Field", action = "Update")]
#[macros::uow_action(entity = "Field", action = "UpdateMulti")]
#[macros::uow_action(entity = "Field", action = "Delete")]
#[macros::uow_action(entity = "Field", action = "DeleteMulti")]
#[macros::uow_action(entity = "Field", action = "GetRelationship")]
#[macros::uow_action(entity = "Field", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Field", action = "SetRelationship")]
#[macros::uow_action(entity = "Field", action = "SetRelationshipMulti")]
pub(in crate::field) trait FieldUnitOfWorkTrait: CommandUnitOfWork {}

pub(in crate::field) trait FieldUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn FieldUnitOfWorkROTrait>;
}

#[macros::uow_action(entity = "Field", action = "GetRO")]
#[macros::uow_action(entity = "Field", action = "GetMultiRO")]
#[macros::uow_action(entity = "Field", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Field", action = "GetRelationshipsFromRightIdsRO")]
pub(in crate::field) trait FieldUnitOfWorkROTrait: QueryUnitOfWork {}
