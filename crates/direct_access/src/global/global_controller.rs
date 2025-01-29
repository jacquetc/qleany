use anyhow::Result;
use common::{database::db_context::DbContext, entities::EntityId};

use super::{
    dtos::{CreateGlobalDto, GlobalDto},
    units_of_work::{GlobalUnitOfWork, GlobalUnitOfWorkRO},
    use_cases::{
        create_global_uc::CreateGlobalUseCase, get_global_uc::GetGlobalUseCase,
        remove_global_uc::RemoveGlobalUseCase, update_global_uc::UpdateGlobalUseCase,
    },
};

pub fn create(db_context: &DbContext, global: &CreateGlobalDto) -> Result<GlobalDto> {
    let mut uow = GlobalUnitOfWork::new(&db_context);
    let mut global_uc = CreateGlobalUseCase::new(&mut uow);
    global_uc.execute(global.clone())
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<GlobalDto>> {
    let uow = GlobalUnitOfWorkRO::new(&db_context);
    let global = GetGlobalUseCase::new(&uow);
    global.execute(id)
}

pub fn update(db_context: &DbContext, global: &GlobalDto) -> Result<GlobalDto> {
    let mut uow = GlobalUnitOfWork::new(&db_context);
    let mut global_uc = UpdateGlobalUseCase::new(&mut uow);
    global_uc.execute(global)
}

pub fn remove(db_context: &DbContext, id: &EntityId) -> Result<()> {
    let mut uow = GlobalUnitOfWork::new(&db_context);
    let mut global_uc = RemoveGlobalUseCase::new(&mut uow);
    global_uc.execute(id)
}
