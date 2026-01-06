pub(super) mod create_use_case_multi_uc;
pub(super) mod create_use_case_uc;
pub(super) mod get_use_case_multi_uc;
pub(super) mod get_use_case_uc;
pub(super) mod remove_use_case_multi_uc;
pub(super) mod remove_use_case_uc;
pub(super) mod update_use_case_multi_uc;
pub(super) mod update_use_case_uc;

pub(super) mod get_use_case_relationship_uc;

pub(super) mod set_use_case_relationship_uc;

use anyhow::Result;
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::UseCase;
use common::types::EntityId;

pub trait UseCaseUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn UseCaseUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "UseCase", action = "Create")]
#[macros::uow_action(entity = "UseCase", action = "CreateMulti")]
#[macros::uow_action(entity = "UseCase", action = "Get")]
#[macros::uow_action(entity = "UseCase", action = "GetMulti")]
#[macros::uow_action(entity = "UseCase", action = "Update")]
#[macros::uow_action(entity = "UseCase", action = "UpdateMulti")]
#[macros::uow_action(entity = "UseCase", action = "Delete")]
#[macros::uow_action(entity = "UseCase", action = "DeleteMulti")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationship")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "UseCase", action = "SetRelationship")]
#[macros::uow_action(entity = "UseCase", action = "SetRelationshipMulti")]
pub trait UseCaseUnitOfWorkTrait: CommandUnitOfWork {}

pub trait UseCaseUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn UseCaseUnitOfWorkROTrait>;
}

#[macros::uow_action(entity = "UseCase", action = "GetRO")]
#[macros::uow_action(entity = "UseCase", action = "GetMultiRO")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationshipsFromRightIdsRO")]
pub trait UseCaseUnitOfWorkROTrait: QueryUnitOfWork {}
