use anyhow::{Ok, Result};
use common::{
    database::db_context::DbContext, entities::EntityId,
};

use super::{
    dtos::{CreateEntityDto, EntityDto},
    units_of_work::{EntityUnitOfWork, EntityUnitOfWorkRO},
    use_cases::{
        create_entity_uc::CreateEntityUseCase, get_entity_uc::GetEntityUseCase,
        remove_entity_uc::RemoveEntityUseCase, update_entity_uc::UpdateEntityUseCase,
    },
};

pub fn create(db_context: &DbContext, entity: &CreateEntityDto) -> Result<EntityDto> {
    let mut uow = EntityUnitOfWork::new(&db_context);
    let mut use_case = CreateEntityUseCase::new(Box::new(uow));
    use_case.execute(entity.clone())
}
pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<EntityDto>> {
    let uow = EntityUnitOfWorkRO::new(&db_context);
    let use_case = GetEntityUseCase::new(Box::new(uow));
    use_case.execute(id)
}

pub fn update(db_context: &DbContext, entity: &EntityDto) -> Result<EntityDto> {
    let mut uow = EntityUnitOfWork::new(&db_context);
    let mut use_case = UpdateEntityUseCase::new(Box::new(uow));
    use_case.execute(entity)
}

pub fn remove(db_context: &DbContext, id: &EntityId) -> Result<()> {

    // delete entity
    let mut uow = EntityUnitOfWork::new(&db_context);
    let mut use_case = RemoveEntityUseCase::new(Box::new(uow));
    use_case.execute(id)?;

    Ok(())
}