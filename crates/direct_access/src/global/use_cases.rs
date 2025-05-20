pub(super) mod create_global_multi_uc;
pub(super) mod create_global_uc;
pub(super) mod get_global_multi_uc;
pub(super) mod get_global_uc;
pub(super) mod remove_global_multi_uc;
pub(super) mod remove_global_uc;
pub(super) mod update_global_multi_uc;
pub(super) mod update_global_uc;

use anyhow::Result;
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::Global;
use common::types::EntityId;

pub(in crate::global) trait GlobalUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn GlobalUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Global", action = "Create")]
#[macros::uow_action(entity = "Global", action = "CreateMulti")]
#[macros::uow_action(entity = "Global", action = "Get")]
#[macros::uow_action(entity = "Global", action = "GetMulti")]
#[macros::uow_action(entity = "Global", action = "Update")]
#[macros::uow_action(entity = "Global", action = "UpdateMulti")]
#[macros::uow_action(entity = "Global", action = "Delete")]
#[macros::uow_action(entity = "Global", action = "DeleteMulti")]
pub(in crate::global) trait GlobalUnitOfWorkTrait: CommandUnitOfWork {}

pub(in crate::global) trait GlobalUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn GlobalUnitOfWorkROTrait>;
}

#[macros::uow_action(entity = "Global", action = "GetRO")]
#[macros::uow_action(entity = "Global", action = "GetMultiRO")]
pub(in crate::global) trait GlobalUnitOfWorkROTrait: QueryUnitOfWork {}
