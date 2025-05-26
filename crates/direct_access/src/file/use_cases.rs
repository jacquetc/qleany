pub(super) mod create_file_multi_uc;
pub(super) mod create_file_uc;
pub(super) mod get_file_multi_uc;
pub(super) mod get_file_uc;
pub(super) mod remove_file_multi_uc;
pub(super) mod remove_file_uc;
pub(super) mod update_file_multi_uc;
pub(super) mod update_file_uc;

use anyhow::Result;
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::entities::File;
use common::types::EntityId;

pub(in crate::file) trait FileUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn FileUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "File", action = "Create")]
#[macros::uow_action(entity = "File", action = "CreateMulti")]
#[macros::uow_action(entity = "File", action = "Get")]
#[macros::uow_action(entity = "File", action = "GetMulti")]
#[macros::uow_action(entity = "File", action = "Update")]
#[macros::uow_action(entity = "File", action = "UpdateMulti")]
#[macros::uow_action(entity = "File", action = "Delete")]
#[macros::uow_action(entity = "File", action = "DeleteMulti")]
pub(in crate::file) trait FileUnitOfWorkTrait: CommandUnitOfWork {}

pub(in crate::file) trait FileUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn FileUnitOfWorkROTrait>;
}

#[macros::uow_action(entity = "File", action = "GetRO")]
#[macros::uow_action(entity = "File", action = "GetMultiRO")]
pub(in crate::file) trait FileUnitOfWorkROTrait: QueryUnitOfWork {}
