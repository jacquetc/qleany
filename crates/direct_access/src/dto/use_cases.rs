pub(super) mod create_dto_multi_uc;
pub(super) mod create_dto_uc;
pub(super) mod get_dto_multi_uc;
pub(super) mod get_dto_relationship_uc;
pub(super) mod get_dto_uc;
pub(super) mod remove_dto_multi_uc;
pub(super) mod remove_dto_uc;
pub(super) mod set_dto_relationship_uc;
pub(super) mod update_dto_multi_uc;
pub(super) mod update_dto_uc;

use anyhow::Result;
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::Dto;
use common::types::EntityId;

pub(in crate::dto) trait DtoUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn DtoUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Dto", action = "Create")]
#[macros::uow_action(entity = "Dto", action = "CreateMulti")]
#[macros::uow_action(entity = "Dto", action = "Get")]
#[macros::uow_action(entity = "Dto", action = "GetMulti")]
#[macros::uow_action(entity = "Dto", action = "Update")]
#[macros::uow_action(entity = "Dto", action = "UpdateMulti")]
#[macros::uow_action(entity = "Dto", action = "Delete")]
#[macros::uow_action(entity = "Dto", action = "DeleteMulti")]
#[macros::uow_action(entity = "Dto", action = "GetRelationship")]
#[macros::uow_action(entity = "Dto", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Dto", action = "SetRelationship")]
#[macros::uow_action(entity = "Dto", action = "SetRelationshipMulti")]
pub(in crate::dto) trait DtoUnitOfWorkTrait: CommandUnitOfWork {}

pub(in crate::dto) trait DtoUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn DtoUnitOfWorkROTrait>;
}

#[macros::uow_action(entity = "Dto", action = "GetRO")]
#[macros::uow_action(entity = "Dto", action = "GetMultiRO")]
#[macros::uow_action(entity = "Dto", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Dto", action = "GetRelationshipsFromRightIdsRO")]
pub(in crate::dto) trait DtoUnitOfWorkROTrait: QueryUnitOfWork {}
