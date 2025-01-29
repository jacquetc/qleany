use anyhow::Result;
use common::{database::db_context::DbContext, entities::EntityId};

use super::{
    dtos::{CreateGlobalDto, GlobalDto},
    units_of_work::{GlobalUnitOfWorkFactory, GlobalUnitOfWorkROFactory},
    use_cases::{
        create_global_uc::CreateGlobalUseCase, get_global_uc::GetGlobalUseCase,
        remove_global_uc::RemoveGlobalUseCase, update_global_uc::UpdateGlobalUseCase,
    },
};

pub fn create(db_context: &DbContext, global: &CreateGlobalDto) -> Result<GlobalDto> {
    let uow_factory = GlobalUnitOfWorkFactory::new(&db_context);
    let mut global_uc = CreateGlobalUseCase::new(Box::new(uow_factory));
    global_uc.execute(global.clone())
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<GlobalDto>> {
    let uow_factory = GlobalUnitOfWorkROFactory::new(&db_context);
    let global = GetGlobalUseCase::new(Box::new(uow_factory));
    global.execute(id)
}

pub fn update(db_context: &DbContext, global: &GlobalDto) -> Result<GlobalDto> {
    let uow_factory = GlobalUnitOfWorkFactory::new(&db_context);
    let mut global_uc = UpdateGlobalUseCase::new(Box::new(uow_factory));
    global_uc.execute(global)
}

pub fn remove(db_context: &DbContext, id: &EntityId) -> Result<()> {
    let uow_factory = GlobalUnitOfWorkFactory::new(&db_context);
    let mut global_uc = RemoveGlobalUseCase::new(Box::new(uow_factory));
    global_uc.execute(id)
}
