use anyhow::{Ok, Result};
use common::{
    database::db_context::DbContext, entities::EntityId,
};

use super::{
    dtos::{CreateUseCaseDto, UseCaseDto},
    units_of_work::{UseCaseUnitOfWorkFactory, UseCaseUnitOfWorkROFactory},
    use_cases::{
        create_use_case_uc::CreateUseCaseUseCase, get_use_case_uc::GetUseCaseUseCase,
        remove_use_case_uc::RemoveUseCaseUseCase, update_use_case_uc::UpdateUseCaseUseCase,
    },
};

pub fn create(db_context: &DbContext, use_case: &CreateUseCaseDto) -> Result<UseCaseDto> {
    let mut uow_factory = UseCaseUnitOfWorkFactory::new(&db_context);
    let mut use_case_uc = CreateUseCaseUseCase::new(Box::new(uow_factory));
    use_case_uc.execute(use_case.clone())
}
pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<UseCaseDto>> {
    let uow_factory = UseCaseUnitOfWorkROFactory::new(&db_context);
    let use_case = GetUseCaseUseCase::new(Box::new(uow_factory));
    use_case.execute(id)
}

pub fn update(db_context: &DbContext, use_case: &UseCaseDto) -> Result<UseCaseDto> {
    let mut uow_factory = UseCaseUnitOfWorkFactory::new(&db_context);
    let mut use_case_uc = UpdateUseCaseUseCase::new(Box::new(uow_factory));
    use_case_uc.execute(use_case)
}

pub fn remove(db_context: &DbContext, id: &EntityId) -> Result<()> {

    // delete use case
    let mut uow_factory = UseCaseUnitOfWorkFactory::new(&db_context);
    let mut use_case = RemoveUseCaseUseCase::new(Box::new(uow_factory));
    use_case.execute(id)?;

    Ok(())
}